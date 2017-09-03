use super::charsetprober::{CharsetProber, filter_international_words};
use super::enums::{ProbingState, SequenceLikelihood, CharacterCategory};

#[derive(Debug)]
pub struct SBStateMachineModel<'a> {
    pub char_to_order_map: &'a [u8],
    pub precedence_matrix: &'a [u8],
    pub typical_positive_ratio: f32,
    pub keep_english_letter: bool,
    pub charset_name: &'a str,
    pub language: &'a str,
}

pub struct SingleByteCharsetProber<'a> {
    c_sample_size: usize,
    c_sb_enough_rel_threshold: usize,
    c_positive_shortcut_threshold: f32,
    c_negative_shortcut_threshold: f32,
    m_state: ProbingState,
    m_model: &'a SBStateMachineModel<'a>,
    m_reversed: bool,
    m_last_order: u8,
    m_seq_counters: [usize; SequenceLikelihood::CATEGORIES],
    m_total_seqs: usize,
    m_total_char: usize,
    m_freq_char: usize,
}

impl<'a> SingleByteCharsetProber<'a> {
    pub fn new(model: &'a SBStateMachineModel, reversed: bool) -> SingleByteCharsetProber<'a> {
        SingleByteCharsetProber {
            c_sample_size: 64,
            c_sb_enough_rel_threshold: 1024,
            c_positive_shortcut_threshold: 0.95,
            c_negative_shortcut_threshold: 0.05,
            m_state: ProbingState::Detecting,
            m_model: model,
            m_reversed: reversed,
            m_last_order: 255,
            m_seq_counters: [0; SequenceLikelihood::CATEGORIES],
            m_total_seqs: 0,
            m_total_char: 0,
            m_freq_char: 0,
        }
    }
}
impl<'a> CharsetProber for SingleByteCharsetProber<'a> {
    fn reset(&mut self) {
        self.m_state = ProbingState::Detecting;
        self.m_last_order = 255;
        self.m_seq_counters = [0; SequenceLikelihood::CATEGORIES];
        self.m_total_seqs = 0;
        self.m_total_char = 0;
        self.m_freq_char = 0;
    }
    fn feed(&mut self, byte_str: &Vec<u8>) -> &ProbingState {
        let byte_str2;
        let byte_str_len;
        if !self.m_model.keep_english_letter {
            byte_str2 = filter_international_words(byte_str);
            byte_str_len = byte_str2.len();
        } else {
            byte_str2 = Vec::new();
            byte_str_len = byte_str.len();
        }
        if byte_str_len <= 0 {
            return &self.m_state;
        }
        let char_to_order_map = self.m_model.char_to_order_map;
        for i in 0..byte_str_len {
            let ch = if self.m_model.keep_english_letter {
                byte_str[i]
            } else {
                byte_str2[i]
            };
            let order = char_to_order_map[ch as usize];
            if order < CharacterCategory::CONTROL {
                self.m_total_char += 1;
            }
            if (order as usize) < self.c_sample_size {
                self.m_freq_char += 1;
                if (self.m_last_order as usize) < self.c_sample_size {
                    self.m_total_seqs += 1;
                    let model;
                    if self.m_reversed {
                        model = self.m_model.precedence_matrix[(order as usize) *
                                                                   self.c_sample_size +
                                                                   (self.m_last_order as usize)];
                    } else {
                        model = self.m_model.precedence_matrix[(self.m_last_order as usize) *
                                                                   self.c_sample_size +
                                                                   (order as usize)];
                    }
                    self.m_seq_counters[model as usize] += 1;
                }
            }
            self.m_last_order = order;
        }
        if self.m_state == ProbingState::Detecting {
            if self.m_total_seqs > self.c_sb_enough_rel_threshold {
                let confidence = self.get_confidence();
                if confidence > self.c_positive_shortcut_threshold {
                    self.m_state = ProbingState::FoundIt;
                } else if confidence < self.c_negative_shortcut_threshold {
                    self.m_state = ProbingState::NotMe;
                }
            }
        }
        &self.m_state
    }
    fn get_charset(&self) -> String {
        self.m_model.charset_name.to_string()
    }
    fn get_confidence(&self) -> f32 {
        let mut r: f32 = 0.01;
        if self.m_total_seqs > 0 {
            r = (self.m_seq_counters[SequenceLikelihood::POSITIVE] as f32) /
                (self.m_total_seqs as f32) / self.m_model.typical_positive_ratio;
            r = r * (self.m_freq_char as f32) / (self.m_total_char as f32);
            if r >= 1.0 {
                r = 0.99;
            }
        }
        r
    }
    fn get_language(&self) -> String {
        self.m_model.language.to_string()
    }
    fn get_state(&self) -> &ProbingState {
        &self.m_state
    }
}
