use super::enums::ProbingState;
use super::charsetprober::{CharsetProber, filter_with_english_letters};

const FREQ_CAT_NUM:usize = 4;

const UDF:u8 = 0;  // undefined
const OTH:u8 = 1;  // other
const ASC:u8 = 2;  // ascii capital letter
const ASS:u8 = 3;  // ascii small letter
const ACV:u8 = 4;  // accent capital vowel
const ACO:u8 = 5;  // accent capital other
const ASV:u8 = 6;  // accent small vowel
const ASO:u8 = 7;  // accent small other
const CLASS_NUM:usize = 8;  // total classes

#[allow(non_upper_case_globals)]
const Latin1_CharToClass:&[u8] = &[
    OTH, OTH, OTH, OTH, OTH, OTH, OTH, OTH,   // 00 - 07
    OTH, OTH, OTH, OTH, OTH, OTH, OTH, OTH,   // 08 - 0F
    OTH, OTH, OTH, OTH, OTH, OTH, OTH, OTH,   // 10 - 17
    OTH, OTH, OTH, OTH, OTH, OTH, OTH, OTH,   // 18 - 1F
    OTH, OTH, OTH, OTH, OTH, OTH, OTH, OTH,   // 20 - 27
    OTH, OTH, OTH, OTH, OTH, OTH, OTH, OTH,   // 28 - 2F
    OTH, OTH, OTH, OTH, OTH, OTH, OTH, OTH,   // 30 - 37
    OTH, OTH, OTH, OTH, OTH, OTH, OTH, OTH,   // 38 - 3F
    OTH, ASC, ASC, ASC, ASC, ASC, ASC, ASC,   // 40 - 47
    ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC,   // 48 - 4F
    ASC, ASC, ASC, ASC, ASC, ASC, ASC, ASC,   // 50 - 57
    ASC, ASC, ASC, OTH, OTH, OTH, OTH, OTH,   // 58 - 5F
    OTH, ASS, ASS, ASS, ASS, ASS, ASS, ASS,   // 60 - 67
    ASS, ASS, ASS, ASS, ASS, ASS, ASS, ASS,   // 68 - 6F
    ASS, ASS, ASS, ASS, ASS, ASS, ASS, ASS,   // 70 - 77
    ASS, ASS, ASS, OTH, OTH, OTH, OTH, OTH,   // 78 - 7F
    OTH, UDF, OTH, ASO, OTH, OTH, OTH, OTH,   // 80 - 87
    OTH, OTH, ACO, OTH, ACO, UDF, ACO, UDF,   // 88 - 8F
    UDF, OTH, OTH, OTH, OTH, OTH, OTH, OTH,   // 90 - 97
    OTH, OTH, ASO, OTH, ASO, UDF, ASO, ACO,   // 98 - 9F
    OTH, OTH, OTH, OTH, OTH, OTH, OTH, OTH,   // A0 - A7
    OTH, OTH, OTH, OTH, OTH, OTH, OTH, OTH,   // A8 - AF
    OTH, OTH, OTH, OTH, OTH, OTH, OTH, OTH,   // B0 - B7
    OTH, OTH, OTH, OTH, OTH, OTH, OTH, OTH,   // B8 - BF
    ACV, ACV, ACV, ACV, ACV, ACV, ACO, ACO,   // C0 - C7
    ACV, ACV, ACV, ACV, ACV, ACV, ACV, ACV,   // C8 - CF
    ACO, ACO, ACV, ACV, ACV, ACV, ACV, OTH,   // D0 - D7
    ACV, ACV, ACV, ACV, ACV, ACO, ACO, ACO,   // D8 - DF
    ASV, ASV, ASV, ASV, ASV, ASV, ASO, ASO,   // E0 - E7
    ASV, ASV, ASV, ASV, ASV, ASV, ASV, ASV,   // E8 - EF
    ASO, ASO, ASV, ASV, ASV, ASV, ASV, OTH,   // F0 - F7
    ASV, ASV, ASV, ASV, ASV, ASO, ASO, ASO,   // F8 - FF
];

// 0 : illegal
// 1 : very unlikely
// 2 : normal
// 3 : very likely
#[allow(non_upper_case_globals)]
const Latin1ClassModel:&[u8] = &[
// UDF OTH ASC ASS ACV ACO ASV ASO
    0,  0,  0,  0,  0,  0,  0,  0,  // UDF
    0,  3,  3,  3,  3,  3,  3,  3,  // OTH
    0,  3,  3,  3,  3,  3,  3,  3,  // ASC
    0,  3,  3,  3,  1,  1,  3,  3,  // ASS
    0,  3,  3,  3,  1,  2,  1,  2,  // ACV
    0,  3,  3,  3,  3,  3,  3,  3,  // ACO
    0,  3,  1,  3,  1,  1,  1,  3,  // ASV
    0,  3,  1,  3,  1,  1,  3,  3,  // ASO
];

pub struct Latin1Prober {
    m_state: ProbingState,
    m_last_char_class: u8,
    m_freq_counter: [usize;FREQ_CAT_NUM],
}

impl Latin1Prober {
    pub fn new() -> Latin1Prober {
        Latin1Prober {
            m_state: ProbingState::Detecting,
            m_last_char_class: OTH,
            m_freq_counter: [0;FREQ_CAT_NUM],
        }
    }
}

impl CharsetProber for Latin1Prober {
    fn reset(&mut self) {
        self.m_state = ProbingState::Detecting;
        self.m_last_char_class = OTH;
        self.m_freq_counter = [0;FREQ_CAT_NUM];
    }
    fn feed(&mut self, byte_str: &[u8]) -> &ProbingState {
        let byte_str2 = filter_with_english_letters(byte_str);
        for c in byte_str2 {
            let char_class = Latin1_CharToClass[c as usize];
            let freq = Latin1ClassModel[(self.m_last_char_class as usize)*CLASS_NUM+(char_class as usize)];
            if freq == 0 {
                self.m_state = ProbingState::NotMe;
                break;
            }
            self.m_freq_counter[freq as usize] += 1;
            self.m_last_char_class = char_class;
        }
        &self.m_state
    }
    fn get_charset(&self) -> String {
        "ISO-8859-1".to_string()
    }
    fn get_confidence(&self) -> f32 {
        if self.m_state == ProbingState::NotMe {
            return 0.01;
        }
        let mut confidence:f32;
        let total:usize = self.m_freq_counter.iter().sum();
        if total < 1 {
            confidence = 0.0;
        } else {
            confidence = ((self.m_freq_counter[3] as f32) - (self.m_freq_counter[1] as f32) * 20.0) / (total as f32);
        }
        if confidence < 0.0 {
            confidence = 0.0;
        }
        confidence * 0.73
    }
    fn get_language(&self) -> String {
        "".to_string()
    }
    fn get_state(&self) -> &ProbingState {
        &self.m_state
    }
}
