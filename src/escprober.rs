use super::enums::{ProbingState, MachineState};
use super::codingstatemachine::CodingStateMachine;
use super::charsetprober::CharsetProber;
use super::escsm::*;

pub struct EscCharsetProber<'a> {
    pub m_state: ProbingState,
    pub m_coding_sm: Vec<CodingStateMachine<'a>>,
    pub m_coding_sm_valid: Vec<bool>,
    pub m_detected_charset: String,
    pub m_detected_language: String,
    pub m_active_sm_count: i16,
}

impl<'a> EscCharsetProber<'a> {
    pub fn new() -> EscCharsetProber<'a> {
        let mut x = EscCharsetProber {
            m_state: ProbingState::Detecting,
            m_coding_sm: Vec::new(),
            m_coding_sm_valid: Vec::new(),
            m_detected_charset: "".to_string(),
            m_detected_language: "".to_string(),
            m_active_sm_count: 0,
        };
        x.m_coding_sm.push(CodingStateMachine::new(&HZ_SM_MODEL));
        x.m_coding_sm_valid.push(true);
        x.m_coding_sm.push(CodingStateMachine::new(&ISO2022CN_SM_MODEL));
        x.m_coding_sm_valid.push(true);
        x.m_coding_sm.push(CodingStateMachine::new(&ISO2022JP_SM_MODEL));
        x.m_coding_sm_valid.push(true);
        x.m_coding_sm.push(CodingStateMachine::new(&ISO2022KR_SM_MODEL));
        x.m_coding_sm_valid.push(true);
        let count = x.m_coding_sm.len();
        x.m_active_sm_count = count as i16;
        x
    }
}

impl<'a> CharsetProber for EscCharsetProber<'a> {
    fn reset(&mut self) {
        self.m_state = ProbingState::Detecting;
        for i in 0..self.m_coding_sm.len() {
            self.m_coding_sm[i].reset();
            self.m_coding_sm_valid[i] = true;
        }
        let count = self.m_coding_sm.len();
        self.m_active_sm_count = count as i16;
        self.m_detected_charset = "".to_string();
        self.m_detected_language = "".to_string();
    }
    fn feed(&mut self, byte_str: &[u8]) -> &ProbingState {
        for c in byte_str {
            for i in 0..self.m_coding_sm.len() {
                if !self.m_coding_sm_valid[i] {
                    continue;
                }
                let sm = &mut self.m_coding_sm[i];
                match sm.next_state(*c) {
                    MachineState::ERROR => {
                        self.m_coding_sm_valid[i] = false;
                        self.m_active_sm_count -= 1;
                        if self.m_active_sm_count <= 0 {
                            self.m_state = ProbingState::NotMe;
                            return &self.m_state;
                        }
                    }
                    MachineState::ITS_ME => {
                        self.m_state = ProbingState::FoundIt;
                        self.m_detected_charset = sm.get_coding_state_machine().to_string();
                        self.m_detected_language = sm.get_language().to_string();
                        return &self.m_state;
                    }
                    _ => {}
                }
            }
        }
        return &self.m_state;
    }
    fn get_charset(&self) -> String {
        self.m_detected_charset.clone()
    }
    fn get_confidence(&self) -> f32 {
        if self.m_detected_charset != "" {
            0.99
        } else {
            0.00
        }
    }
    fn get_language(&self) -> String {
        self.m_detected_language.clone()
    }
    fn get_state(&self) -> &ProbingState {
        &self.m_state
    }
}
