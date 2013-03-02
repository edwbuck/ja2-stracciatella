#include "TranslationTable.h"

unsigned char const g_en_TranslationTable[TRANSLATION_TABLE_SIZE] =
{
	[L'A'] =   0,
	[L'B'] =   1,
	[L'C'] =   2,
	[L'D'] =   3,
	[L'E'] =   4,
	[L'F'] =   5,
	[L'G'] =   6,
	[L'H'] =   7,
	[L'I'] =   8,
	[L'J'] =   9,
	[L'K'] =  10,
	[L'L'] =  11,
	[L'M'] =  12,
	[L'N'] =  13,
	[L'O'] =  14,
	[L'P'] =  15,
	[L'Q'] =  16,
	[L'R'] =  17,
	[L'S'] =  18,
	[L'T'] =  19,
	[L'U'] =  20,
	[L'V'] =  21,
	[L'W'] =  22,
	[L'X'] =  23,
	[L'Y'] =  24,
	[L'Z'] =  25,

	[L'a'] =  26,
	[L'b'] =  27,
	[L'c'] =  28,
	[L'd'] =  29,
	[L'e'] =  30,
	[L'f'] =  31,
	[L'g'] =  32,
	[L'h'] =  33,
	[L'i'] =  34,
	[L'j'] =  35,
	[L'k'] =  36,
	[L'l'] =  37,
	[L'm'] =  38,
	[L'n'] =  39,
	[L'o'] =  40,
	[L'p'] =  41,
	[L'q'] =  42,
	[L'r'] =  43,
	[L's'] =  44,
	[L't'] =  45,
	[L'u'] =  46,
	[L'v'] =  47,
	[L'w'] =  48,
	[L'x'] =  49,
	[L'y'] =  50,
	[L'z'] =  51,

	[L'0'] =  52,
	[L'1'] =  53,
	[L'2'] =  54,
	[L'3'] =  55,
	[L'4'] =  56,
	[L'5'] =  57,
	[L'6'] =  58,
	[L'7'] =  59,
	[L'8'] =  60,
	[L'9'] =  61,

	[L'!'] =  62,
	[L'@'] =  63,
	[L'#'] =  64,
	[L'$'] =  65,
	[L'%'] =  66,
	[L'^'] =  67,
	[L'&'] =  68,
	[L'*'] =  69,
	[L'('] =  70,
	[L')'] =  71,
	[L'-'] =  72,
	[L'_'] =  73,
	[L'+'] =  74,
	[L'='] =  75,
	[L'|'] =  76,
	[L'\\'] =  77,
	[L'{'] =  78,
	[L'}'] =  79,
	[L'['] =  80,
	[L']'] =  81,
	[L':'] =  82,
	[L';'] =  83,
	[L'"'] =  84,
	[L'\''] =  85,
	[L'<'] =  86,
	[L'>'] =  87,
	[L','] =  88,
	[L'.'] =  89,
	[L'?'] =  90,
	[L'/'] =  91,
	[L' '] =  92,
	[196] =  93, // A umlaut
	[214] =  94, // O umlaut
	[220] =  95, // U umlaut
	[228] =  96, // a umlaut
	[246] =  97, // o umlaut
	[252] =  98, // u umlaut
	[223] =  99, // sharp s

	// duplicate 196, // Ä
	[192] = 133, // À
	[193] = 134, // Á
	[194] = 135, // Â
	[199] = 136, // Ç
	[203] = 137, // Ë
	[200] = 138, // È
	[201] = 139, // É				140
	[202] = 140, // Ê
	[207] = 141, // Ï
	// duplicate 214, // Ö
	[210] = 143, // Ò
	[211] = 144, // Ó
	[212] = 145, // Ô
	// duplicate 220, // Ü
	[217] = 147, // Ù
	[218] = 148, // Ú
	[219] = 149, // Û				150
	// duplicate 228, // ä
	[224] = 151, // à
	[225] = 152, // á
	[226] = 153, // â
	[231] = 154, // ç
	[235] = 155, // ë
	[232] = 156, // è
	[233] = 157, // é
	[234] = 158, // ê
	[239] = 159, // ï				160
	// duplicate 246, // ö
	[242] = 161, // ò
	[243] = 162, // ó
	[244] = 163, // ô
	// duplicate 252, // ü
	[249] = 165, // ù
	[250] = 166, // ú
	[251] = 167, // û

	[0x00CC] = 168, // Ì
	[0x00EC] = 169, // ì
	[0x0104] = 170, // Ą
	[0x0106] = 171, // Ć
	[0x0118] = 172, // Ę
	[0x0141] = 173, // Ł
	[0x0143] = 174, // Ń
	//[0x00D3] = 175, // Ó (duplicate)
	[0x015A] = 176, // Ś
	[0x017B] = 177, // Ż
	[0x0179] = 178, // Ź
	[0x0105] = 179, // ą
	[0x0107] = 180, // ć
	[0x0119] = 181, // ę
	[0x0142] = 182, // ł
	[0x0144] = 183, // ń
	//[0x00F3] = 184, // ó (duplicate)
	[0x015B] = 185, // ś
	[0x017C] = 186, // ż
	[0x017A] = 187, // ź

	[0x0410] = 100, // cyrillic A
	[0x0411] = 101, // cyrillic BE
	[0x0412] = 102, // cyrillic VE
	[0x0413] = 103, // cyrillic GHE
	[0x0414] = 104, // cyrillic DE
	[0x0415] = 105, // cyrillic IE
	[0x0401] = 106, // cyrillic IO
	[0x0416] = 107, // cyrillic ZHE
	[0x0417] = 108, // cyrillic ZE
	[0x0418] = 109, // cyrillic I
	[0x0419] = 110, // cyrillic short I
	[0x041A] = 111, // cyrillic KA
	[0x041B] = 112, // cyrillic EL
	[0x041C] = 113, // cyrillic EM
	[0x041D] = 114, // cyrillic EN
	[0x041E] = 115, // cyrillic O
	[0x041F] = 116, // cyrillic PE
	[0x0420] = 117, // cyrillic ER
	[0x0421] = 118, // cyrillic ES
	[0x0422] = 119, // cyrillic TE
	[0x0423] = 120, // cyrillic U
	[0x0424] = 121, // cyrillic EF
	[0x0425] = 122, // cyrillic HA
	[0x0426] = 123, // cyrillic TSE
	[0x0427] = 124, // cyrillic CHE
	[0x0428] = 125, // cyrillic SHA
	[0x0429] = 126, // cyrillic SHCHA
	[0x042A] = 128, // cyrillic capital hard sign, glyph is missing, mapped to soft sign
	[0x042B] = 127, // cyrillic YERU
	[0x042C] = 128, // cyrillic capital soft sign
	[0x042D] = 129, // cyrillic E
	[0x042E] = 130, // cyrillic YU
	[0x042F] = 131, // cyrillic YA

	/* There are no lowercase cyrillic glyphs in the fonts (at least neither in
	 * the german nor the polish datafiles), so reuse the uppercase cyrillic
	 * glyphs. */
	[0x0430] = 100, // cyrillic a
	[0x0431] = 101, // cyrillic be
	[0x0432] = 102, // cyrillic ve
	[0x0433] = 103, // cyrillic ghe
	[0x0434] = 104, // cyrillic de
	[0x0435] = 105, // cyrillic ie
	[0x0451] = 106, // cyrillic io
	[0x0436] = 107, // cyrillic zhe
	[0x0437] = 108, // cyrillic ze
	[0x0438] = 109, // cyrillic i
	[0x0439] = 110, // cyrillic short i
	[0x043A] = 111, // cyrillic ka
	[0x043B] = 112, // cyrillic el
	[0x043C] = 113, // cyrillic em
	[0x043D] = 114, // cyrillic en
	[0x043E] = 115, // cyrillic o
	[0x043F] = 116, // cyrillic pe
	[0x0440] = 117, // cyrillic er
	[0x0441] = 118, // cyrillic es
	[0x0442] = 119, // cyrillic te
	[0x0443] = 120, // cyrillic u
	[0x0444] = 121, // cyrillic ef
	[0x0445] = 122, // cyrillic ha
	[0x0446] = 123, // cyrillic tse
	[0x0447] = 124, // cyrillic che
	[0x0448] = 125, // cyrillic sha
	[0x0449] = 126, // cyrillic shcha
	[0x044A] = 128, // cyrillic lowercase hard sign, glyph is missing, mapped to soft sign
	[0x044B] = 127, // cyrillic yeru
	[0x044C] = 128, // cyrillic lowercase soft sign
	[0x044D] = 129, // cyrillic e
	[0x044E] = 130, // cyrillic yu
	[0x044F] = 131  // cyrillic ya
};