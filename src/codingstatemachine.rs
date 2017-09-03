use super::enums::MachineState;
use super::mbcssm::StateMachineModel;

#[derive(Debug)]
pub struct CodingStateMachine<'a> {
    m_model: &'a StateMachineModel<'a>,
    m_curr_byte_pos: usize,
    m_curr_char_len: usize,
    m_curr_state: u8,
}

impl<'a> CodingStateMachine<'a> {
    pub fn new(sm: &'a StateMachineModel) -> CodingStateMachine<'a> {
        CodingStateMachine {
            m_model: sm,
            m_curr_byte_pos: 0,
            m_curr_char_len: 0,
            m_curr_state: MachineState::START,
        }
    }
    pub fn reset(&mut self) {
        self.m_curr_state = MachineState::START;
    }
    pub fn next_state(&mut self, c: u8) -> u8 {
        let byte_class = self.m_model.class_table[c as usize];
        if self.m_curr_state == MachineState::START {
            self.m_curr_byte_pos = 0;
            self.m_curr_char_len = self.m_model.char_len_table[byte_class as usize] as usize;
        }
        let curr_state = self.m_curr_state * self.m_model.class_factor + byte_class;
        self.m_curr_state = self.m_model.state_table[curr_state as usize];
        self.m_curr_byte_pos += 1;
        self.m_curr_state
    }
    pub fn get_current_charlen(&self) -> usize {
        self.m_curr_char_len
    }
    pub fn get_coding_state_machine(&self) -> &str {
        self.m_model.name
    }
    pub fn get_language(&self) -> &str {
        self.m_model.language
    }
}
