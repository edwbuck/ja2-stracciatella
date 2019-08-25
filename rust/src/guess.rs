//! This module contains code to guess Vanillaversion with resource packs.

use log::{error, info};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashSet;
use std::convert::From;
use std::error::Error;
use std::ffi::OsString;
use std::fmt;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use serde_json;

use crate::config::VanillaVersion;
use crate::res::{
    Resource, ResourceError, ResourcePack, ResourcePackBuilder, ResourcePropertiesExt,
};
use crate::unicode::Nfc;

/// Guess the vanilla version of the resources in the game dir.
pub fn guess_vanilla_version(gamedir: &str) -> Guess {
    let path = Path::new(gamedir);
    let mut logged = Guess::default();
    if let Err(err) = logged.guess_vanilla_version(&path) {
        error!("Error: {}", err.desc);
    }
    logged
}

/// A difference that was detected in resource packs
#[derive(Debug)]
enum Difference {
    OnlyExistsInDataDir(Option<Nfc>, Nfc),
    OnlyExistsInPack(Option<Nfc>, Nfc),
    FileSizeMismatch(Nfc, Option<i64>, Option<i64>),
    HashMismatch(Nfc, String, Option<String>, Option<String>),
}

/// The result of matching a resource pack
#[derive(Debug)]
struct MatchResourcesResult {
    number_of_resources: usize,
    differences: Vec<Difference>,
}

/// Percentages of differences in resource packs
struct Percentages {
    only_exists_in_data_dir: f64,
    only_exists_in_pack: f64,
    file_size_mismatch: f64,
    hash_mismatch: f64,
    total: f64,
}

impl Percentages {
    /// Log percentages with log level info
    fn log(&self) {
        info!(
            "Percentage of resources only in game dir: {}",
            self.only_exists_in_data_dir
        );
        info!(
            "Percentage of resources only in pack: {}",
            self.only_exists_in_pack
        );
        info!(
            "Percentage of resources with file size mismatch: {}",
            self.file_size_mismatch
        );
        info!(
            "Percentage of resources with hash mismatch: {}",
            self.hash_mismatch
        );
        info!("Accumulated difference: {}", self.total);
    }
}

impl From<&MatchResourcesResult> for Percentages {
    fn from(result: &MatchResourcesResult) -> Self {
        let number_of_resources = result.number_of_resources;
        let count_differences = |filter: &Fn(&&Difference) -> bool| {
            result.differences.iter().filter(filter).count()
        };
        let num_only_in_datadir = count_differences(&|d| match d {
            Difference::OnlyExistsInDataDir(_, _) => true,
            _ => false,
        });
        let percentage_only_in_datadir = num_only_in_datadir as f64 / number_of_resources as f64;
        let num_only_in_pack = count_differences(&|d| match d {
            Difference::OnlyExistsInPack(_, _) => true,
            _ => false,
        });
        let percentage_only_in_pack = num_only_in_pack as f64 / number_of_resources as f64;
        let num_file_size_mismatch = count_differences(&|d| match d {
            Difference::FileSizeMismatch(_, _, _) => true,
            _ => false,
        });
        let percentage_file_size_mismatch =
            num_file_size_mismatch as f64 / number_of_resources as f64;
        let num_hash_mismatch = count_differences(&|d| match d {
            Difference::HashMismatch(_, _, _, _) => true,
            _ => false,
        });
        let percentage_hash_mismatch = num_hash_mismatch as f64 / number_of_resources as f64;
        let total = percentage_only_in_datadir
            + percentage_only_in_pack
            + percentage_file_size_mismatch
            + percentage_hash_mismatch;

        Percentages {
            only_exists_in_data_dir: percentage_only_in_datadir,
            only_exists_in_pack: percentage_only_in_pack,
            file_size_mismatch: percentage_file_size_mismatch,
            hash_mismatch: percentage_hash_mismatch,
            total,
        }
    }
}

/// The result of guessing game version for data directory
#[derive(Default)]
pub struct Guess {
    pub vanilla_version: Option<VanillaVersion>,
}

type GuessResult<T> = Result<T, GuessError>;

impl Guess {
    /// Guess version by inspecting archives for language specific paths
    fn guess_by_language_specific_resources(
        &self,
        datadir: &Path,
    ) -> GuessResult<Option<VanillaVersion>> {
        info!("Getting resources with_archive_slf");
        let resources = ResourcePackBuilder::new()
            .with_path(&datadir, &datadir)
            .with_archive("slf")
            .execute("paths")?
            .resources;
        info!("Inspecting resource paths for language specific files");
        let mut num_dutch = 0;
        let mut num_german = 0;
        let mut num_italian = 0;
        let mut num_polish = 0;
        let mut num_russian = 0;
        for resource in resources {
            let p = Nfc::caseless(&resource.path);
            if p.starts_with("dutch/") {
                num_dutch += 1;
            } else if p.starts_with("german/") {
                num_german += 1;
            } else if p.starts_with("italian/") {
                num_italian += 1;
            } else if p.starts_with("polish/") {
                num_polish += 1;
            } else if p.starts_with("russian/") {
                num_russian += 1;
            }
        }

        let version = match (num_dutch, num_german, num_italian, num_polish, num_russian) {
            (n, 0, 0, 0, 0) if n > 0 => Some(VanillaVersion::DUTCH),
            (0, n, 0, 0, 0) if n > 0 => Some(VanillaVersion::GERMAN),
            (0, 0, n, 0, 0) if n > 0 => Some(VanillaVersion::ITALIAN),
            (0, 0, 0, n, 0) if n > 0 => Some(VanillaVersion::POLISH),
            (0, 0, 0, 0, n) if n > 0 => Some(VanillaVersion::RUSSIAN),
            _ => None,
        };

        match version {
            Some(version) => info!(
                "Detected exclusively language specific files for language {:?}",
                version
            ),
            None => info!("Detected no language specific files or multiple different languages"),
        }

        Ok(version)
    }

    /// Guess version by building resource pack and comparing it to other resource
    /// packs in externalized directory
    fn guess_by_resource_matching(&self, datadir: &Path) -> GuessResult<Option<VanillaVersion>> {
        let results = self
            .get_pack_paths()?
            .par_iter()
            .map(|path| self.compare_pack(datadir, path))
            .collect::<Result<Vec<_>, _>>()?;

        let (best_version, best_difference) = results.iter().fold(
            (None, std::f64::MAX),
            |(best_version, best_difference), (version, match_resources)| {
                info!("Match statistics with vanilla_version {:?}", version);
                let percentages: Percentages = match_resources.into();
                percentages.log();

                if percentages.total < best_difference {
                    (Some(version), percentages.total)
                } else {
                    (best_version, best_difference)
                }
            },
        );

        Ok(best_version.map(|version| {
            if best_difference == 0. {
                info!("Found perfect match with vanilla_version {:?}", version);
            } else {
                info!(
                    "Using vanilla_version {:?} with least difference {}",
                    version, best_difference
                );
            }
            *version
        }))
    }

    /// Guess vanilla version for game directory
    fn guess_vanilla_version(&mut self, gamedir: &Path) -> GuessResult<()> {
        let datadir = self.get_datadir(gamedir)?;

        if let Some(version) = self.guess_by_language_specific_resources(&datadir)? {
            self.vanilla_version = Some(version);
            return Ok(());
        }

        if let Some(version) = self.guess_by_resource_matching(&datadir)? {
            self.vanilla_version = Some(version);
            return Ok(());
        }

        Err("Give up".to_owned().into())
    }

    /// Compare resource pack and data directory
    fn compare_pack(
        &self,
        datadir: &Path,
        pack_path: &Path,
    ) -> GuessResult<(VanillaVersion, MatchResourcesResult)> {
        let pack = self.get_pack(&pack_path)?;
        let version = self.get_version(&pack)?;
        let match_resources = self.match_resources(&pack, &datadir)?;
        Ok((version, match_resources))
    }

    /// Find data directory within game directory
    fn get_datadir(&self, gamedir: &Path) -> GuessResult<PathBuf> {
        info!("Looking for data dir in {:?}", &gamedir);
        let data_caseless = Nfc::caseless("data");
        let mut paths: Vec<PathBuf> = gamedir
            .read_dir()?
            .filter_map(|x| {
                if let Ok(entry) = x {
                    if let Some(file_name) = entry.file_name().to_str() {
                        if Nfc::caseless(file_name) == data_caseless {
                            let path = entry.path();
                            if path.is_dir() {
                                return Some(path);
                            }
                        }
                    }
                }
                None
            })
            .collect();
        if paths.len() > 1 {
            return Err(format!("Too many data dirs: {:?}", paths).into());
        }
        if paths.is_empty() {
            return Err("Data dir not found".to_owned().into());
        }
        let path = paths.remove(0);
        info!("Found {:?}", &path);
        Ok(path)
    }

    /// Find all resource packs in externalized directory
    fn get_pack_paths(&self) -> GuessResult<Vec<PathBuf>> {
        let dir = Path::new("externalized/resource_packs");
        info!("Searching for resource packs in {:?}", &dir);
        let paths: Vec<PathBuf> = dir
            .read_dir()?
            .filter_map(|x| {
                if let Ok(entry) = x {
                    let path = entry.path();
                    if is_json_file(&path) {
                        info!("Found {:?}", &path);
                        return Some(path);
                    }
                }
                None
            })
            .collect();
        Ok(paths)
    }

    /// Read resource pack json
    fn get_pack(&self, path: &Path) -> GuessResult<ResourcePack> {
        let f = File::open(&path)?;
        let pack: ResourcePack = serde_json::from_reader(f)?;
        Ok(pack)
    }

    /// Get vanilla version from resource pack
    fn get_version(&self, pack: &ResourcePack) -> GuessResult<VanillaVersion> {
        if let Some(version) = pack.get_str("vanilla_version") {
            let version = VanillaVersion::from_str(version)?;
            return Ok(version);
        }
        Err("vanilla_version is missing".to_owned().into())
    }

    /// Match resources in pack with data directory
    fn match_resources(
        &self,
        pack: &ResourcePack,
        datadir: &Path,
    ) -> GuessResult<MatchResourcesResult> {
        let mut builder = ResourcePackBuilder::new();
        builder.with_path(&datadir, &datadir);
        if pack.has_file_size() {
            builder.with_file_size();
        }
        for archive in pack.get_archives() {
            builder.with_archive(&archive);
        }
        let hashes = pack.get_hashes();
        for hash in &hashes {
            builder.with_hash(hash);
        }
        let resources = builder.clone().execute("guess")?.resources;
        let get_compared_path = |resource: &Resource| {
            (
                resource.get_str("archive_path").map(Nfc::caseless),
                Nfc::caseless(&resource.path),
            )
        };

        info!("Comparing resource pack paths for \"{}\"", &pack.name);
        let datadir_paths: HashSet<_> = resources.iter().map(get_compared_path).collect();
        let pack_paths: HashSet<_> = pack.resources.iter().map(get_compared_path).collect();
        let all_paths: HashSet<_> = datadir_paths.union(&pack_paths).collect();
        let additional_paths_datadir =
            datadir_paths
                .difference(&pack_paths)
                .map(|(archive_path, path)| {
                    Difference::OnlyExistsInDataDir(archive_path.to_owned(), path.clone())
                });
        let addional_paths_pack =
            pack_paths
                .difference(&datadir_paths)
                .map(|(archive_path, path)| {
                    Difference::OnlyExistsInPack(archive_path.to_owned(), path.clone())
                });
        let common_paths: HashSet<_> = datadir_paths.intersection(&pack_paths).collect();
        let resources: Vec<_> = resources
            .into_iter()
            .filter(move |r| common_paths.contains(&get_compared_path(r)))
            .collect();

        let file_comparison_result = resources.iter().flat_map(|resource| {
            let pack_resource = pack
                .resources
                .iter()
                .find(|r| Nfc::caseless(&r.path) == Nfc::caseless(&resource.path))
                .expect("was in intersection of resources");

            let resource_file_size = resource.get_i64("file_size");
            let pack_file_size = pack_resource.get_i64("file_size");
            if resource_file_size != pack_file_size {
                return vec![Difference::FileSizeMismatch(
                    Nfc::caseless(&resource.path),
                    resource_file_size,
                    pack_file_size,
                )];
            }

            hashes
                .iter()
                .filter_map(|hash| {
                    let prop = "hash_".to_owned() + hash;
                    let resource_hash = resource.get_str(&prop);
                    let pack_hash = pack_resource.get_str(&prop);

                    if resource_hash != pack_hash {
                        return Some(Difference::HashMismatch(
                            Nfc::caseless(&resource.path),
                            hash.clone(),
                            resource_hash.map(|s| s.to_string()),
                            pack_hash.map(|s| s.to_string()),
                        ));
                    }
                    None
                })
                .collect()
        });

        Ok(MatchResourcesResult {
            number_of_resources: all_paths.len(),
            differences: additional_paths_datadir
                .chain(addional_paths_pack)
                .chain(file_comparison_result)
                .collect(),
        })
    }
}

fn is_json_file(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }
    path.extension().unwrap_or(&OsString::new()).to_str() == Some("json")
}

#[derive(Debug)]
struct GuessError {
    desc: String,
}

impl Error for GuessError {
    fn description(&self) -> &str {
        &self.desc
    }
}

impl fmt::Display for GuessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GuessError({})", self.description())
    }
}

impl From<String> for GuessError {
    fn from(desc: String) -> Self {
        GuessError { desc }
    }
}

impl From<io::Error> for GuessError {
    fn from(err: io::Error) -> Self {
        GuessError {
            desc: format!("{:?}", err),
        }
    }
}

impl From<ResourceError> for GuessError {
    fn from(err: ResourceError) -> Self {
        GuessError {
            desc: format!("{:?}", err),
        }
    }
}

impl From<serde_json::Error> for GuessError {
    fn from(err: serde_json::Error) -> Self {
        GuessError {
            desc: format!("{:?}", err),
        }
    }
}
