use std::ops::Deref;
use std::ops::DerefMut;
use super::enums::MachineState;
use super::mbcharsetprober::MultiByteCharsetProber;
use super::charsetprober::CharsetProber;
use super::enums::ProbingState;
use super::codingstatemachine::CodingStateMachine;
use super::mbcssm::SJIS_SM_MODEL;
use super::chardistribution::SJISDistributionAnalysis;
use super::jpcntx::{JapaneseContextAnalysis, SJISContextAnalysis};

pub struct SJISProber<'a> {
    base: MultiByteCharsetProber<'a>,
    m_context_analyzer: SJISContextAnalysis,
}

impl<'x> Deref for SJISProber<'x> {
    type Target = MultiByteCharsetProber<'x>;
    fn deref<'a>(&'a self) -> &'a MultiByteCharsetProber<'x> {
        &self.base
    }
}
impl<'x> DerefMut for SJISProber<'x> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut MultiByteCharsetProber<'x> {
        &mut self.base
    }
}

impl<'a> CharsetProber for SJISProber<'a> {
    fn reset(&mut self) {
        self.base.reset();
        self.m_context_analyzer.reset();
    }
    fn feed(&mut self, byte_str: &[u8]) -> &ProbingState {
        {
            let sm = self.base.m_coding_sm.as_mut().unwrap();
            let da = self.base.m_distribution_analyzer.as_mut().unwrap();
            for i in 0..byte_str.len() {
                match sm.next_state(byte_str[i]) {
                    MachineState::START => {
                        let char_len = sm.get_current_charlen();
                        if i == 0 {
                            self.base.m_last_char[1] = byte_str[0];
                            self.m_context_analyzer.feed(
                                &self.base.m_last_char[(2 - char_len) as
                                                           usize..],
                                char_len as usize,
                            );
                            da.feed(&self.base.m_last_char[..], char_len);
                        } else {
                            self.m_context_analyzer.feed(
                                &byte_str[i + 1 -
                                              char_len as usize..],
                                char_len as usize,
                            );
                            da.feed(&byte_str[i - 1..i + 1], char_len);
                        }
                    }
                    MachineState::ERROR => {
                        self.base.m_state = ProbingState::NotMe;
                        break;
                    }
                    MachineState::ITS_ME => {
                        self.base.m_state = ProbingState::FoundIt;
                        break;
                    }
                    _ => {}
                }
            }
        }
        self.base.m_last_char[0] = byte_str[byte_str.len() - 1];
        if self.base.m_state == ProbingState::Detecting {
            if (self.m_context_analyzer.got_enough_data()) && (self.get_confidence() > 0.95) {
                self.base.m_state = ProbingState::FoundIt;
            }
        }
        &self.base.m_state
    }
    fn get_charset(&self) -> String {
        self.m_context_analyzer.get_charset()
    }
    fn get_confidence(&self) -> f32 {
        let a = self.base.get_confidence();
        let b = self.m_context_analyzer.get_confidence();
        if a>b {
            a
        } else {
            b
        }
    }
    fn get_language(&self) -> String {
        "Japanese".to_string()
    }
    fn get_state(&self) -> &ProbingState {
        self.base.get_state()
    }
}

impl<'a> SJISProber<'a> {
    pub fn new() -> SJISProber<'a> {
        let mut x = SJISProber {
            base: MultiByteCharsetProber::new(),
            m_context_analyzer: SJISContextAnalysis::new(),
        };
        x.base.m_coding_sm = Some(CodingStateMachine::new(&SJIS_SM_MODEL));
        x.base.m_distribution_analyzer = Some(Box::new(SJISDistributionAnalysis::new()));
        x
    }
}
