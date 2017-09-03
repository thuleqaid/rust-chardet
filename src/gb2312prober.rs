use std::ops::Deref;
use std::ops::DerefMut;
use super::mbcharsetprober::MultiByteCharsetProber;
use super::charsetprober::CharsetProber;
use super::enums::ProbingState;
use super::codingstatemachine::CodingStateMachine;
use super::mbcssm::GB2312_SM_MODEL;
use super::chardistribution::GB2312DistributionAnalysis;

pub struct GB2312Prober<'a> {
    base: MultiByteCharsetProber<'a>,
}

impl<'x> Deref for GB2312Prober<'x> {
    type Target = MultiByteCharsetProber<'x>;
    fn deref<'a>(&'a self) -> &'a MultiByteCharsetProber<'x> {
        &self.base
    }
}
impl<'x> DerefMut for GB2312Prober<'x> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut MultiByteCharsetProber<'x> {
        &mut self.base
    }
}

impl<'a> CharsetProber for GB2312Prober<'a> {
    fn reset(&mut self) {
        self.base.reset();
    }
    fn feed(&mut self, byte_str: &Vec<u8>) -> &ProbingState {
        self.base.feed(byte_str)
    }
    fn get_charset(&self) -> String {
        "GB2312".to_string()
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

impl<'a> GB2312Prober<'a> {
    pub fn new() -> GB2312Prober<'a> {
        let mut x = GB2312Prober { base:MultiByteCharsetProber::new() };
        x.base.m_coding_sm = Some(CodingStateMachine::new(&GB2312_SM_MODEL));
        x.base.m_distribution_analyzer = Some(Box::new(GB2312DistributionAnalysis::new()));
        x
    }
}
