use super::sbcharsetprober::SBStateMachineModel;

// 255: Control characters that usually does not exist in any text
// 254: Carriage/Return
// 253: symbol (punctuation) that does not belong to word
// 252: 0 - 9

// Character Mapping Table:
#[allow(non_upper_case_globals)]
const Latin5_TurkishCharToOrderMap:&[u8] = &[
255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,
255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,
255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,
255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,
255, 23, 37, 47, 39, 29, 52, 36, 45, 53, 60, 16, 49, 20, 46, 42,
 48, 69, 44, 35, 31, 51, 38, 62, 65, 43, 56,255,255,255,255,255,
255,  1, 21, 28, 12,  2, 18, 27, 25,  3, 24, 10,  5, 13,  4, 15,
 26, 64,  7,  8,  9, 14, 32, 57, 58, 11, 22,255,255,255,255,255,
180,179,178,177,176,175,174,173,172,171,170,169,168,167,166,165,
164,163,162,161,160,159,101,158,157,156,155,154,153,152,151,106,
150,149,148,147,146,145,144,100,143,142,141,140,139,138,137,136,
 94, 80, 93,135,105,134,133, 63,132,131,130,129,128,127,126,125,
124,104, 73, 99, 79, 85,123, 54,122, 98, 92,121,120, 91,103,119,
 68,118,117, 97,116,115, 50, 90,114,113,112,111, 55, 41, 40, 86,
 89, 70, 59, 78, 71, 82, 88, 33, 77, 66, 84, 83,110, 75, 61, 96,
 30, 67,109, 74, 87,102, 34, 95, 81,108, 76, 72, 17,  6, 19,107,
];

#[allow(non_upper_case_globals)]
const TurkishLangModel:&[u8] = &[
3,2,3,3,3,1,3,3,3,3,3,3,3,3,2,1,1,3,3,1,3,3,0,3,3,3,3,3,0,3,1,3,
3,2,1,0,0,1,1,0,0,0,1,0,0,1,1,1,1,0,0,0,0,0,0,0,2,2,0,0,1,0,0,1,
3,2,2,3,3,0,3,3,3,3,3,3,3,2,3,1,0,3,3,1,3,3,0,3,3,3,3,3,0,3,0,3,
3,1,1,0,1,0,1,0,0,0,0,0,0,1,1,1,1,0,0,0,0,0,0,0,2,2,0,0,0,1,0,1,
3,3,2,3,3,0,3,3,3,3,3,3,3,2,3,1,1,3,3,0,3,3,1,2,3,3,3,3,0,3,0,3,
3,1,1,0,0,0,1,0,0,0,0,1,1,0,1,2,1,0,0,0,1,0,0,0,0,2,0,0,0,0,0,1,
3,3,3,3,3,3,2,3,3,3,3,3,3,3,3,1,3,3,2,0,3,2,1,2,2,1,3,3,0,0,0,2,
2,2,0,1,0,0,1,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1,1,0,1,0,0,1,
3,3,3,2,3,3,1,2,3,3,3,3,3,3,3,1,3,2,1,0,3,2,0,1,2,3,3,2,1,0,0,2,
2,1,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,2,0,2,0,0,0,
1,0,1,3,3,1,3,3,3,3,3,3,3,1,2,0,0,2,3,0,2,3,0,0,2,2,2,3,0,3,0,1,
2,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,0,3,3,3,0,3,2,0,2,3,2,3,3,1,0,0,2,
3,2,0,0,1,0,0,0,0,0,0,2,0,0,1,0,0,0,0,0,0,0,0,0,1,1,1,0,2,0,0,1,
3,3,3,2,3,3,2,3,3,3,3,2,3,3,3,0,3,3,0,0,2,1,0,0,2,3,2,2,0,0,0,2,
2,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,1,0,1,0,2,0,0,1,
3,3,3,2,3,3,3,3,3,3,3,2,3,3,3,0,3,2,0,1,3,2,1,1,3,2,3,2,1,0,0,2,
2,2,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1,0,0,0,0,0,
3,3,3,2,3,3,3,3,3,3,3,2,3,3,3,0,3,2,2,0,2,3,0,0,2,2,2,2,0,0,0,2,
3,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,2,0,1,0,0,0,
3,3,3,3,3,3,3,2,2,2,2,3,2,3,3,0,3,3,1,1,2,2,0,0,2,2,3,2,0,0,1,3,
0,3,1,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1,0,0,0,0,1,
3,3,3,2,3,3,3,2,1,2,2,3,2,3,3,0,3,2,0,0,1,1,0,1,1,2,1,2,0,0,0,1,
0,3,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1,0,1,0,0,0,
3,3,3,2,3,3,2,3,2,2,2,3,3,3,3,1,3,1,1,0,3,2,1,1,3,3,2,3,1,0,0,1,
1,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,2,0,0,1,
3,2,2,3,3,0,3,3,3,3,3,3,3,2,2,1,0,3,3,1,3,3,0,1,3,3,2,3,0,3,0,3,
2,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,
2,2,2,3,3,0,3,3,3,3,3,3,3,3,3,0,0,3,2,0,3,3,0,3,2,3,3,3,0,3,1,3,
2,0,0,0,0,0,0,0,0,0,0,1,0,1,2,0,1,0,0,0,0,0,0,0,2,2,0,0,1,0,0,1,
3,3,3,1,2,3,3,1,0,0,1,0,0,3,3,2,3,0,0,2,0,0,2,0,2,0,0,0,2,0,2,0,
0,3,1,0,1,0,0,0,2,2,1,0,1,1,2,1,2,2,2,0,2,1,1,0,0,0,2,0,0,0,0,0,
1,2,1,3,3,0,3,3,3,3,3,2,3,0,0,0,0,2,3,0,2,3,1,0,2,3,1,3,0,3,0,2,
3,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
3,3,3,1,3,3,2,2,3,2,2,0,1,2,3,0,1,2,1,0,1,0,0,0,1,0,2,2,0,0,0,1,
1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,1,1,0,0,1,0,0,0,
3,3,3,1,3,3,1,1,3,3,1,1,3,3,1,0,2,1,2,0,2,1,0,0,1,1,2,1,0,0,0,2,
2,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
3,3,3,1,0,2,1,3,0,0,2,0,0,3,3,0,3,0,0,1,0,1,2,0,0,1,1,2,2,0,1,0,
0,1,2,1,1,0,1,0,1,1,1,1,1,0,1,1,1,2,2,1,2,0,1,0,0,0,0,0,0,1,0,0,
3,3,3,2,3,2,3,3,0,2,2,2,3,3,3,0,3,0,0,0,2,2,0,1,2,1,1,1,0,0,0,1,
0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,
3,3,3,3,3,3,2,1,2,2,3,3,3,3,2,0,2,0,0,0,2,2,0,0,2,1,3,3,0,0,1,1,
1,1,0,0,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1,0,0,0,
1,1,2,3,3,0,3,3,3,3,3,3,2,2,0,2,0,2,3,2,3,2,2,2,2,2,2,2,1,3,2,3,
2,0,2,1,2,2,2,2,1,1,2,2,1,2,2,1,2,0,0,2,1,1,0,2,1,0,0,1,0,0,0,1,
2,3,3,1,1,1,0,1,1,1,2,3,2,1,1,0,0,0,0,0,0,0,0,0,0,1,0,1,0,0,0,0,
0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
3,3,3,2,2,2,3,2,3,2,2,1,3,3,3,0,2,1,2,0,2,1,0,0,1,1,1,1,1,0,0,1,
2,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,2,0,1,0,0,0,
3,3,3,2,3,3,3,3,3,2,3,1,2,3,3,1,2,0,0,0,0,0,0,0,3,2,1,1,0,0,0,0,
2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,
3,3,3,2,2,3,3,2,1,1,1,1,1,3,3,0,3,1,0,0,1,1,0,0,3,1,2,1,0,0,0,0,
0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,
3,3,3,2,2,3,2,2,2,3,2,1,1,3,3,0,3,0,0,0,0,1,0,0,3,1,1,2,0,0,0,1,
1,0,0,1,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
1,1,1,3,3,0,3,3,3,3,3,2,2,2,1,2,0,2,1,2,2,1,1,0,1,2,2,2,2,2,2,2,
0,0,2,1,2,1,2,1,0,1,1,3,1,2,1,1,2,0,0,2,0,1,0,1,0,1,0,0,0,1,0,1,
3,3,3,1,3,3,3,0,1,1,0,2,2,3,1,0,3,0,0,0,1,0,0,0,1,0,0,1,0,1,0,0,
1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
3,3,2,0,0,2,2,1,0,0,1,0,0,3,3,1,3,0,0,1,1,0,2,0,3,0,0,0,2,0,1,1,
0,1,2,0,1,2,2,0,2,2,2,2,1,0,2,1,1,0,2,0,2,1,2,0,0,0,0,0,0,0,0,0,
3,3,3,1,3,2,3,2,0,2,2,2,1,3,2,0,2,1,2,0,1,2,0,0,1,0,2,2,0,0,0,2,
1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,1,0,0,0,
3,3,3,0,3,3,1,1,2,3,1,0,3,2,3,0,3,0,0,0,1,0,0,0,1,0,1,0,0,0,0,0,
1,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
0,0,0,3,3,0,3,3,2,3,3,2,2,0,0,0,0,1,2,0,1,3,0,0,0,3,1,1,0,3,0,2,
2,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
3,3,3,1,2,2,1,0,3,1,1,1,1,3,3,2,3,0,0,1,0,1,2,0,2,2,0,2,2,0,2,1,
0,2,2,1,1,1,1,0,2,1,1,0,1,1,1,1,2,1,2,1,2,0,1,0,1,0,0,0,0,0,0,0,
3,3,3,0,1,1,3,0,0,1,1,0,0,2,2,0,3,0,0,1,1,0,1,0,0,0,0,0,2,0,0,0,
0,3,1,0,1,0,1,0,2,0,0,1,0,1,0,1,1,1,2,1,1,0,2,0,0,0,0,0,0,0,0,0,
3,3,3,0,2,0,2,0,1,1,1,0,0,3,3,0,2,0,0,1,0,0,2,1,1,0,1,0,1,0,1,0,
0,2,0,1,2,0,2,0,2,1,1,0,1,0,2,1,1,0,2,1,1,0,1,0,0,0,1,1,0,0,0,0,
3,2,3,0,1,0,0,0,0,0,0,0,0,1,2,0,1,0,0,1,0,0,1,0,0,0,0,0,2,0,0,0,
0,0,1,1,0,0,1,0,1,0,0,1,0,0,0,2,1,0,1,0,2,0,0,0,0,0,0,0,0,0,0,0,
3,3,3,0,0,2,3,0,0,1,0,1,0,2,3,2,3,0,0,1,3,0,2,1,0,0,0,0,2,0,1,0,
0,2,1,0,0,1,1,0,2,1,0,0,1,0,0,1,1,0,1,1,2,0,1,0,0,0,0,1,0,0,0,0,
3,2,2,0,0,1,1,0,0,0,0,0,0,3,1,1,1,0,0,0,0,0,1,0,0,0,0,0,2,0,1,0,
0,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,0,0,1,0,1,0,0,0,0,0,0,0,0,0,
0,0,0,3,3,0,2,3,2,2,1,2,2,1,1,2,0,1,3,2,2,2,0,0,2,2,0,0,0,1,2,1,
3,0,2,1,1,0,1,1,1,0,1,2,2,2,1,1,2,0,0,0,0,1,0,1,1,0,0,0,0,0,0,0,
0,1,1,2,3,0,3,3,3,2,2,2,2,1,0,1,0,1,0,1,2,2,0,0,2,2,1,3,1,1,2,1,
0,0,1,1,2,0,1,1,0,0,1,2,0,2,1,1,2,0,0,1,0,0,0,1,0,1,0,1,0,0,0,0,
3,3,2,0,0,3,1,0,0,0,0,0,0,3,2,1,2,0,0,1,0,0,2,0,0,0,0,0,2,0,1,0,
0,2,1,1,0,0,1,0,1,2,0,0,1,1,0,0,2,1,1,1,1,0,2,0,0,0,0,0,0,0,0,0,
3,3,2,0,0,1,0,0,0,0,1,0,0,3,3,2,2,0,0,1,0,0,2,0,1,0,0,0,2,0,1,0,
0,0,1,1,0,0,2,0,2,1,0,0,1,1,2,1,2,0,2,1,2,1,1,1,0,0,1,1,0,0,0,0,
3,3,2,0,0,2,2,0,0,0,1,1,0,2,2,1,3,1,0,1,0,1,2,0,0,0,0,0,1,0,1,0,
0,1,1,0,0,0,0,0,1,0,0,1,0,0,0,1,1,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0,
3,3,3,2,0,0,0,1,0,0,1,0,0,2,3,1,2,0,0,1,0,0,2,0,0,0,1,0,2,0,2,0,
0,1,1,2,2,1,2,0,2,1,1,0,0,1,1,0,1,1,1,1,2,1,1,0,0,0,0,0,0,0,0,0,
3,3,3,0,2,1,2,1,0,0,1,1,0,3,3,1,2,0,0,1,0,0,2,0,2,0,1,1,2,0,0,0,
0,0,1,1,1,1,2,0,1,1,0,1,1,1,1,0,0,0,1,1,1,0,1,0,0,0,1,0,0,0,0,0,
3,3,3,0,2,2,3,2,0,0,1,0,0,2,3,1,0,0,0,0,0,0,2,0,2,0,0,0,2,0,0,0,
0,1,1,0,0,0,1,0,0,1,0,1,1,0,1,0,1,1,1,0,1,0,0,0,0,0,0,0,0,0,0,0,
3,2,3,0,0,0,0,0,0,0,1,0,0,2,2,2,2,0,0,1,0,0,2,0,0,0,0,0,2,0,1,0,
0,0,2,1,1,0,1,0,2,1,1,0,0,1,1,2,1,0,2,0,2,0,1,0,0,0,2,0,0,0,0,0,
0,0,0,2,2,0,2,1,1,1,1,2,2,0,0,1,0,1,0,0,1,3,0,0,0,0,1,0,0,2,1,0,
0,0,1,0,1,0,0,0,0,0,2,1,0,1,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,
2,0,0,2,3,0,2,3,1,2,2,0,2,0,0,2,0,2,1,1,1,2,1,0,0,1,2,1,1,2,1,0,
1,0,2,0,1,0,1,1,0,0,2,2,1,2,1,1,2,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,
3,3,3,0,2,1,2,0,0,0,1,0,0,3,2,0,1,0,0,1,0,0,2,0,0,0,1,2,1,0,1,0,
0,0,0,0,1,0,1,0,0,1,0,0,0,0,1,0,1,0,1,1,1,0,1,0,0,0,0,0,0,0,0,0,
0,0,0,2,2,0,2,2,1,1,0,1,1,1,1,1,0,0,1,2,1,1,1,0,1,0,0,0,1,1,1,1,
0,0,2,1,0,1,1,1,0,1,1,2,1,2,1,1,2,0,1,1,2,1,0,2,0,0,0,0,0,0,0,0,
3,2,2,0,0,2,0,0,0,0,0,0,0,2,2,0,2,0,0,1,0,0,2,0,0,0,0,0,2,0,0,0,
0,2,1,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,0,0,1,0,1,0,0,0,0,0,0,0,0,0,
0,0,0,3,2,0,2,2,0,1,1,0,1,0,0,1,0,0,0,1,0,1,0,0,0,0,0,1,0,0,0,0,
2,0,1,0,1,0,1,1,0,0,1,2,0,1,0,1,1,0,0,1,0,1,0,2,0,0,0,0,0,0,0,0,
2,2,2,0,1,1,0,0,0,1,0,0,0,1,2,0,1,0,0,1,0,0,1,0,0,0,0,1,2,0,1,0,
0,0,1,0,0,0,1,0,0,1,0,0,0,0,0,0,1,0,1,0,2,0,0,0,0,0,0,0,0,0,0,0,
2,2,2,2,1,0,1,1,1,0,0,0,0,1,2,0,0,1,0,0,0,1,0,0,1,0,0,0,0,0,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,
1,1,2,0,1,0,0,0,1,0,1,0,0,0,1,0,0,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,2,0,0,0,0,0,1,
0,0,1,2,2,0,2,1,2,1,1,2,2,0,0,0,0,1,0,0,1,1,0,0,2,0,0,0,0,1,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,
2,2,2,0,0,0,1,0,0,0,0,0,0,2,2,1,1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,
0,0,0,0,1,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
0,0,0,1,1,0,0,0,1,0,0,0,1,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
2,2,2,0,1,0,1,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,1,0,1,0,0,0,0,0,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,1,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
0,0,1,0,0,0,0,0,0,0,0,0,0,2,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
];

#[allow(non_upper_case_globals)]
pub static Latin5TurkishModel:SBStateMachineModel = SBStateMachineModel {
    char_to_order_map: Latin5_TurkishCharToOrderMap,
    precedence_matrix: TurkishLangModel,
    typical_positive_ratio: 0.970290,
    keep_english_letter: true,
    charset_name: "ISO-8859-9",
    language: "Turkish",
};

