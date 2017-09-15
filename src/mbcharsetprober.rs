use super::enums::{ProbingState, MachineState};
use super::codingstatemachine::CodingStateMachine;
use super::chardistribution::CharDistributionAnalysis;
use super::charsetprober::CharsetProber;

pub struct MultiByteCharsetProber<'a> {
    pub m_state: ProbingState,
    pub m_distribution_analyzer: Option<Box<CharDistributionAnalysis>>,
    pub m_coding_sm: Option<CodingStateMachine<'a>>,
    pub m_last_char: [u8; 2],
}

impl<'a> MultiByteCharsetProber<'a> {
    pub fn new() -> MultiByteCharsetProber<'a> {
        MultiByteCharsetProber {
            m_state: ProbingState::Detecting,
            m_distribution_analyzer: None,
            m_coding_sm: None,
            m_last_char: [0, 0],
        }
    }
}

impl<'a> CharsetProber for MultiByteCharsetProber<'a> {
    fn reset(&mut self) {
        self.m_state = ProbingState::Detecting;
        self.m_last_char = [0, 0];
        if self.m_coding_sm.is_some() {
            let x = self.m_coding_sm.as_mut().unwrap();
            x.reset();
        }
        if self.m_distribution_analyzer.is_some() {
            let x = self.m_distribution_analyzer.as_mut().unwrap();
            x.reset();
        }
    }
    fn feed(&mut self, byte_str: &[u8]) -> &ProbingState {
        let enoughdata:bool;
        {
            let sm = self.m_coding_sm.as_mut().unwrap();
            let da = self.m_distribution_analyzer.as_mut().unwrap();
            for i in 0..byte_str.len() {
                match sm.next_state(byte_str[i]) {
                    MachineState::START => {
                        let char_len = sm.get_current_charlen();
                        if i == 0 {
                            self.m_last_char[1] = byte_str[0];
                            da.feed(&self.m_last_char[..], char_len);
                        } else {
                            da.feed(&byte_str[i - 1..i + 1], char_len);
                        }
                    }
                    MachineState::ERROR => {
                        self.m_state = ProbingState::NotMe;
                        break;
                    }
                    MachineState::ITS_ME => {
                        self.m_state = ProbingState::FoundIt;
                        break;
                    }
                    _ => {}
                }
            }
            enoughdata = da.got_enough_data();
        }
        self.m_last_char[0] = byte_str[byte_str.len() - 1];
        if self.m_state == ProbingState::Detecting {
            if (enoughdata) && (self.get_confidence() > 0.95) {
                self.m_state = ProbingState::FoundIt;
            }
        }
        &self.m_state
    }
    fn get_charset(&self) -> String {
        "".to_string()
    }
    fn get_confidence(&self) -> f32 {
        let da = self.m_distribution_analyzer.as_ref().unwrap();
        da.get_confidence()
    }
    fn get_language(&self) -> String {
        "".to_string()
    }
    fn get_state(&self) -> &ProbingState {
        &self.m_state
    }
}
