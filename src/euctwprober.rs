use std::ops::Deref;
use std::ops::DerefMut;
use super::mbcharsetprober::MultiByteCharsetProber;
use super::charsetprober::CharsetProber;
use super::enums::ProbingState;
use super::codingstatemachine::CodingStateMachine;
use super::mbcssm::EUCTW_SM_MODEL;
use super::chardistribution::EUCTWDistributionAnalysis;

pub struct EUCTWProber<'a> {
    base: MultiByteCharsetProber<'a>,
}

impl<'x> Deref for EUCTWProber<'x> {
    type Target = MultiByteCharsetProber<'x>;
    fn deref<'a>(&'a self) -> &'a MultiByteCharsetProber<'x> {
        &self.base
    }
}
impl<'x> DerefMut for EUCTWProber<'x> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut MultiByteCharsetProber<'x> {
        &mut self.base
    }
}

impl<'a> CharsetProber for EUCTWProber<'a> {
    fn reset(&mut self) {
        self.base.reset();
    }
    fn feed(&mut self, byte_str: &Vec<u8>) -> &ProbingState {
        self.base.feed(byte_str)
    }
    fn get_charset(&self) -> String {
        "EUC-KR".to_string()
    }
    fn get_confidence(&self) -> f32 {
        self.base.get_confidence()
    }
    fn get_language(&self) -> String {
        "Korean".to_string()
    }
    fn get_state(&self) -> &ProbingState {
        self.base.get_state()
    }
}

impl<'a> EUCTWProber<'a> {
    pub fn new() -> EUCTWProber<'a> {
        let mut x = EUCTWProber { base:MultiByteCharsetProber::new() };
        x.base.m_coding_sm = Some(CodingStateMachine::new(&EUCTW_SM_MODEL));
        x.base.m_distribution_analyzer = Some(Box::new(EUCTWDistributionAnalysis::new()));
        x
    }
}
