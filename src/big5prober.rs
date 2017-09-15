use std::ops::Deref;
use std::ops::DerefMut;
use super::mbcharsetprober::MultiByteCharsetProber;
use super::charsetprober::CharsetProber;
use super::enums::ProbingState;
use super::codingstatemachine::CodingStateMachine;
use super::mbcssm::BIG5_SM_MODEL;
use super::chardistribution::Big5DistributionAnalysis;

pub struct Big5Prober<'a> {
    base: MultiByteCharsetProber<'a>,
}

impl<'x> Deref for Big5Prober<'x> {
    type Target = MultiByteCharsetProber<'x>;
    fn deref<'a>(&'a self) -> &'a MultiByteCharsetProber<'x> {
        &self.base
    }
}
impl<'x> DerefMut for Big5Prober<'x> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut MultiByteCharsetProber<'x> {
        &mut self.base
    }
}

impl<'a> CharsetProber for Big5Prober<'a> {
    fn reset(&mut self) {
        self.base.reset();
    }
    fn feed(&mut self, byte_str: &[u8]) -> &ProbingState {
        self.base.feed(byte_str)
    }
    fn get_charset(&self) -> String {
        "Big5".to_string()
    }
    fn get_confidence(&self) -> f32 {
        self.base.get_confidence()
    }
    fn get_language(&self) -> String {
        "Chinese".to_string()
    }
    fn get_state(&self) -> &ProbingState {
        self.base.get_state()
    }
}

impl<'a> Big5Prober<'a> {
    pub fn new() -> Big5Prober<'a> {
        let mut x = Big5Prober { base:MultiByteCharsetProber::new() };
        x.base.m_coding_sm = Some(CodingStateMachine::new(&BIG5_SM_MODEL));
        x.base.m_distribution_analyzer = Some(Box::new(Big5DistributionAnalysis::new()));
        x
    }
}
