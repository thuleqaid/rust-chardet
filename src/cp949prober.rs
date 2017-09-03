use std::ops::Deref;
use std::ops::DerefMut;
use super::mbcharsetprober::MultiByteCharsetProber;
use super::charsetprober::CharsetProber;
use super::enums::ProbingState;
use super::codingstatemachine::CodingStateMachine;
use super::mbcssm::CP949_SM_MODEL;
use super::chardistribution::EUCKRDistributionAnalysis;

pub struct CP949Prober<'a> {
    base: MultiByteCharsetProber<'a>,
}

impl<'x> Deref for CP949Prober<'x> {
    type Target = MultiByteCharsetProber<'x>;
    fn deref<'a>(&'a self) -> &'a MultiByteCharsetProber<'x> {
        &self.base
    }
}
impl<'x> DerefMut for CP949Prober<'x> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut MultiByteCharsetProber<'x> {
        &mut self.base
    }
}

impl<'a> CharsetProber for CP949Prober<'a> {
    fn reset(&mut self) {
        self.base.reset();
    }
    fn feed(&mut self, byte_str: &Vec<u8>) -> &ProbingState {
        self.base.feed(byte_str)
    }
    fn get_charset(&self) -> String {
        "CP949".to_string()
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

impl<'a> CP949Prober<'a> {
    pub fn new() -> CP949Prober<'a> {
        let mut x = CP949Prober { base:MultiByteCharsetProber::new() };
        x.base.m_coding_sm = Some(CodingStateMachine::new(&CP949_SM_MODEL));
        x.base.m_distribution_analyzer = Some(Box::new(EUCKRDistributionAnalysis::new()));
        x
    }
}
