use super::charsetprober::CharsetProber;
use super::enums::{ProbingState, MachineState};
use super::codingstatemachine::CodingStateMachine;
use super::mbcssm::UTF8_SM_MODEL;

pub struct UTF8Prober<'a> {
    pub m_state: ProbingState,
    pub m_coding_sm: CodingStateMachine<'a>,
    pub m_num_mb_chars: i32,
}

impl<'a> UTF8Prober<'a> {
    pub fn new() -> UTF8Prober<'a> {
        UTF8Prober {
            m_state: ProbingState::Detecting,
            m_coding_sm: CodingStateMachine::new(&UTF8_SM_MODEL),
            m_num_mb_chars: 0,
        }
    }
}

impl<'a> CharsetProber for UTF8Prober<'a> {
    fn reset(&mut self) {
        self.m_state = ProbingState::Detecting;
        self.m_coding_sm.reset();
        self.m_num_mb_chars = 0;
    }
    fn feed(&mut self, byte_str: &Vec<u8>) -> &ProbingState {
        for i in 0..byte_str.len() {
            match self.m_coding_sm.next_state(byte_str[i]) {
                MachineState::START => {
                    if self.m_coding_sm.get_current_charlen() >= 2 {
                        self.m_num_mb_chars += 1;
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
        if self.m_state == ProbingState::Detecting {
            if self.get_confidence() > 0.95 {
                self.m_state = ProbingState::FoundIt;
            }
        }
        &self.m_state
    }
    fn get_charset(&self) -> String {
        "utf-8".to_string()
    }
    fn get_confidence(&self) -> f32 {
        if self.m_num_mb_chars < 6 {
            1.0 - 0.99 * (0.5_f32.powi(self.m_num_mb_chars))
        } else {
            0.99
        }
    }
    fn get_language(&self) -> String {
        "".to_string()
    }
    fn get_state(&self) -> &ProbingState {
        &self.m_state
    }
}
