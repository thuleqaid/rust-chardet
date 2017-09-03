use super::charsetprober::{CharsetProber, filter_high_byte_only};
use super::sbcharsetprober::SingleByteCharsetProber;
use super::enums::ProbingState;
use super::langhebrewmodel::Win1255HebrewModel;

#[allow(dead_code)]
pub struct HebrewProber<'a> {
    c_final_kaf: u8,
    c_normal_kaf: u8,
    c_final_mem: u8,
    c_normal_mem: u8,
    c_final_nun: u8,
    c_normal_nun: u8,
    c_final_pe: u8,
    c_normal_pe: u8,
    c_final_tsadi: u8,
    c_normal_tsadi: u8,
    c_min_final_char_distance: u8,
    c_min_model_distance: f32,
    c_visual_hebrew_name: String,
    c_logical_hebrew_name: String,
    m_final_char_logical_score: usize,
    m_final_char_visual_score: usize,
    m_prev: u8,
    m_before_prev: u8,
    m_logical_prober: SingleByteCharsetProber<'a>,
    m_visual_prober: SingleByteCharsetProber<'a>,
    m_state: ProbingState,
}

impl<'a> HebrewProber<'a> {
    pub fn new() -> HebrewProber<'a> {
        HebrewProber {
            c_final_kaf: 0xea,
            c_normal_kaf: 0xeb,
            c_final_mem: 0xed,
            c_normal_mem: 0xee,
            c_final_nun: 0xef,
            c_normal_nun: 0xf0,
            c_final_pe: 0xf3,
            c_normal_pe: 0xf4,
            c_final_tsadi: 0xf5,
            c_normal_tsadi: 0xf6,
            c_min_final_char_distance: 5,
            c_min_model_distance: 0.01,
            c_visual_hebrew_name: "ISO-8859-8".to_string(),
            c_logical_hebrew_name: "windows-1255".to_string(),
            m_final_char_logical_score: 0,
            m_final_char_visual_score: 0,
            m_prev: 0x20,
            m_before_prev: 0x20,
            m_logical_prober: SingleByteCharsetProber::new(&Win1255HebrewModel, false),
            m_visual_prober: SingleByteCharsetProber::new(&Win1255HebrewModel, true),
            m_state: ProbingState::Detecting,
        }
    }
    pub fn is_final(&self, c: u8) -> bool {
        (c == self.c_final_kaf) || (c == self.c_final_mem) || (c == self.c_final_nun) ||
            (c == self.c_final_pe) || (c == self.c_final_tsadi)
    }
    pub fn is_non_final(&self, c: u8) -> bool {
        (c == self.c_normal_kaf) || (c == self.c_normal_mem) || (c == self.c_normal_nun) ||
            (c == self.c_normal_pe)
    }
}

impl<'a> CharsetProber for HebrewProber<'a> {
    fn reset(&mut self) {
        self.m_final_char_logical_score = 0;
        self.m_final_char_visual_score = 0;
        self.m_prev = 0x20;
        self.m_before_prev = 0x20;
        self.m_logical_prober.reset();
        self.m_visual_prober.reset();
        self.m_state = ProbingState::Detecting;
    }
    fn feed(&mut self, byte_str: &Vec<u8>) -> &ProbingState {
        if *self.get_state() == ProbingState::NotMe {
            return self.get_state();
        }
        let byte_str2 = filter_high_byte_only(byte_str);
        for cur in byte_str2 {
            if cur == 0x20 {
                if self.m_before_prev != 0x20 {
                    if self.is_final(self.m_prev) {
                        self.m_final_char_logical_score += 1;
                    } else if self.is_non_final(self.m_prev) {
                        self.m_final_char_visual_score += 1;
                    }
                }
            } else {
                if (self.m_before_prev == 0x20) && (self.is_final(self.m_prev)) && (cur != 0x20) {
                    self.m_final_char_visual_score += 1;
                }
            }
            self.m_before_prev = self.m_prev;
            self.m_prev = cur;
        }
        let s1 = self.m_logical_prober.feed(byte_str);
        let s2 = self.m_visual_prober.feed(byte_str);
        if (*s1 == ProbingState::FoundIt) || (*s2 == ProbingState::FoundIt) {
            self.m_state = ProbingState::FoundIt;
        } else if (*s1 == ProbingState::NotMe) && (*s2 == ProbingState::NotMe) {
            self.m_state = ProbingState::NotMe;
        } else {
            self.m_state = ProbingState::Detecting;
        }
        &self.m_state
    }
    fn get_charset(&self) -> String {
        let finalsub: i32 = (self.m_final_char_logical_score as i32) -
            (self.m_final_char_visual_score as i32);
        if finalsub >= (self.c_min_final_char_distance as i32) {
            return self.c_logical_hebrew_name.clone();
        }
        if finalsub <= -(self.c_min_final_char_distance as i32) {
            return self.c_visual_hebrew_name.clone();
        }
        let modelsub: f32 = self.m_logical_prober.get_confidence() - self.m_visual_prober.get_confidence();
        if modelsub > self.c_min_model_distance {
            return self.c_logical_hebrew_name.clone();
        }
        if modelsub < -self.c_min_model_distance {
            return self.c_visual_hebrew_name.clone();
        }
        if finalsub < 0 {
            self.c_visual_hebrew_name.clone()
        } else {
            self.c_logical_hebrew_name.clone()
        }
    }
    fn get_confidence(&self) -> f32 {
        let c1 = self.m_logical_prober.get_confidence();
        let c2 = self.m_visual_prober.get_confidence();
        if c1 > c2 {
            c1
        } else {
            c2
        }
    }
    fn get_language(&self) -> String {
        "Hebrew".to_string()
    }
    fn get_state(&self) -> &ProbingState {
        if *self.m_logical_prober.get_state() == ProbingState::NotMe {
            self.m_visual_prober.get_state()
        } else if *self.m_logical_prober.get_state() == ProbingState::Detecting {
            if *self.m_visual_prober.get_state() == ProbingState::NotMe {
                self.m_logical_prober.get_state()
            } else {
                self.m_visual_prober.get_state()
            }
        } else {
            self.m_logical_prober.get_state()
        }
    }
}
