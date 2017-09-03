use super::big5freq::*;
use super::euckrfreq::*;
use super::euctwfreq::*;
use super::gb2312freq::*;
use super::jisfreq::*;

pub trait CharDistributionAnalysis {
    fn reset(&mut self);
    fn feed(&mut self, char: &[u8], char_len: usize);
    fn get_confidence(&self) -> f32;
    fn got_enough_data(&self) -> bool;
    fn get_order(&self, char: &[u8]) -> Option<usize>;
}

pub struct BaseCharDistributionAnalysis<'a> {
    c_enough_data_threshold: usize,
    c_minimum_data_threshold: usize,
    c_sure_yes: f32,
    c_sure_no: f32,
    m_char_to_freq_order: Option<&'a [u16]>,
    m_table_size: usize,
    m_typical_distribution_ratio: f32,
    m_done: bool,
    m_total_chars: usize,
    m_freq_chars: usize,
}

impl<'a> BaseCharDistributionAnalysis<'a> {
    pub fn new() -> BaseCharDistributionAnalysis<'a> {
        BaseCharDistributionAnalysis {
            c_enough_data_threshold: 1024,
            c_minimum_data_threshold: 3,
            c_sure_yes: 0.99,
            c_sure_no: 0.01,
            m_char_to_freq_order: None,
            m_table_size: 0,
            m_typical_distribution_ratio: 1.0,
            m_done: false,
            m_total_chars: 0,
            m_freq_chars: 0,
        }
    }
}

impl<'a> CharDistributionAnalysis for BaseCharDistributionAnalysis<'a> {
    fn reset(&mut self) {
        self.m_done = false;
        self.m_total_chars = 0;
        self.m_freq_chars = 0;
    }
    fn get_confidence(&self) -> f32 {
        if (self.m_total_chars <= 0) || (self.m_freq_chars <= self.c_minimum_data_threshold) {
            return self.c_sure_no;
        }
        if self.m_total_chars != self.m_freq_chars {
            let r = self.m_freq_chars as f32 /
                ((self.m_total_chars - self.m_freq_chars) as f32 *
                     self.m_typical_distribution_ratio);
            if r < self.c_sure_yes {
                return r;
            }
        }
        return self.c_sure_yes;
    }
    fn got_enough_data(&self) -> bool {
        self.m_total_chars > self.c_enough_data_threshold
    }
    fn feed(&mut self, _: &[u8], _: usize) {}
    fn get_order(&self, _: &[u8]) -> Option<usize> {
        None
    }
}

pub struct EUCTWDistributionAnalysis<'a> {
    base: BaseCharDistributionAnalysis<'a>,
}

impl<'a> EUCTWDistributionAnalysis<'a> {
    pub fn new() -> EUCTWDistributionAnalysis<'a> {
        let mut x = EUCTWDistributionAnalysis { base: BaseCharDistributionAnalysis::new() };
        x.base.m_char_to_freq_order = Some(EUCTW_CHAR_TO_FREQ_ORDER);
        x.base.m_table_size = EUCTW_TABLE_SIZE;
        x.base.m_typical_distribution_ratio = EUCTW_TYPICAL_DISTRIBUTION_RATIO;
        x
    }
}

impl<'a> CharDistributionAnalysis for EUCTWDistributionAnalysis<'a> {
    fn reset(&mut self) {
        self.base.reset();
    }
    fn get_confidence(&self) -> f32 {
        self.base.get_confidence()
    }
    fn got_enough_data(&self) -> bool {
        self.base.got_enough_data()
    }
    fn feed(&mut self, char: &[u8], char_len: usize) {
        let order: Option<usize>;
        if char_len == 2 {
            order = self.get_order(char);
        } else {
            order = None;
        }
        if order.is_some() {
            self.base.m_total_chars += 1;
            if order.unwrap() < self.base.m_table_size {
                let tmp = self.base.m_char_to_freq_order.unwrap();
                if 512 > tmp[order.unwrap() as usize] {
                    self.base.m_freq_chars += 1;
                }
            }
        }
    }
    fn get_order(&self, char: &[u8]) -> Option<usize> {
        if char.len() >= 2 {
            if char[0] >= 0xC4 {
                let order: isize = 94 * (char[0] as isize - 0xC4) + char[1] as isize - 0xA1;
                if order >= 0 {
                    Some(order as usize)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct EUCKRDistributionAnalysis<'a> {
    base: BaseCharDistributionAnalysis<'a>,
}

impl<'a> EUCKRDistributionAnalysis<'a> {
    pub fn new() -> EUCKRDistributionAnalysis<'a> {
        let mut x = EUCKRDistributionAnalysis { base: BaseCharDistributionAnalysis::new() };
        x.base.m_char_to_freq_order = Some(EUCKR_CHAR_TO_FREQ_ORDER);
        x.base.m_table_size = EUCKR_TABLE_SIZE;
        x.base.m_typical_distribution_ratio = EUCKR_TYPICAL_DISTRIBUTION_RATIO;
        x
    }
}

impl<'a> CharDistributionAnalysis for EUCKRDistributionAnalysis<'a> {
    fn reset(&mut self) {
        self.base.reset();
    }
    fn get_confidence(&self) -> f32 {
        self.base.get_confidence()
    }
    fn got_enough_data(&self) -> bool {
        self.base.got_enough_data()
    }
    fn feed(&mut self, char: &[u8], char_len: usize) {
        let order: Option<usize>;
        if char_len == 2 {
            order = self.get_order(char);
        } else {
            order = None;
        }
        if order.is_some() {
            self.base.m_total_chars += 1;
            if order.unwrap() < self.base.m_table_size {
                let tmp = self.base.m_char_to_freq_order.unwrap();
                if 512 > tmp[order.unwrap() as usize] {
                    self.base.m_freq_chars += 1;
                }
            }
        }
    }
    fn get_order(&self, char: &[u8]) -> Option<usize> {
        if char.len() >= 2 {
            if char[0] >= 0xB0 {
                let order: isize = 94 * (char[0] as isize - 0xB0) + char[1] as isize - 0xA1;
                if order >= 0 {
                    Some(order as usize)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct GB2312DistributionAnalysis<'a> {
    base: BaseCharDistributionAnalysis<'a>,
}

impl<'a> GB2312DistributionAnalysis<'a> {
    pub fn new() -> GB2312DistributionAnalysis<'a> {
        let mut x = GB2312DistributionAnalysis { base: BaseCharDistributionAnalysis::new() };
        x.base.m_char_to_freq_order = Some(GB2312_CHAR_TO_FREQ_ORDER);
        x.base.m_table_size = GB2312_TABLE_SIZE;
        x.base.m_typical_distribution_ratio = GB2312_TYPICAL_DISTRIBUTION_RATIO;
        x
    }
}

impl<'a> CharDistributionAnalysis for GB2312DistributionAnalysis<'a> {
    fn reset(&mut self) {
        self.base.reset();
    }
    fn get_confidence(&self) -> f32 {
        self.base.get_confidence()
    }
    fn got_enough_data(&self) -> bool {
        self.base.got_enough_data()
    }
    fn feed(&mut self, char: &[u8], char_len: usize) {
        let order: Option<usize>;
        if char_len == 2 {
            order = self.get_order(char);
        } else {
            order = None;
        }
        if order.is_some() {
            self.base.m_total_chars += 1;
            if order.unwrap() < self.base.m_table_size {
                let tmp = self.base.m_char_to_freq_order.unwrap();
                if 512 > tmp[order.unwrap() as usize] {
                    self.base.m_freq_chars += 1;
                }
            }
        }
    }
    fn get_order(&self, char: &[u8]) -> Option<usize> {
        if char.len() >= 2 {
            if (char[0] >= 0xB0) && (char[1] >= 0xA1) {
                let order: isize = 94 * (char[0] as isize - 0xB0) + char[1] as isize - 0xA1;
                if order >= 0 {
                    Some(order as usize)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct Big5DistributionAnalysis<'a> {
    base: BaseCharDistributionAnalysis<'a>,
}

impl<'a> Big5DistributionAnalysis<'a> {
    pub fn new() -> Big5DistributionAnalysis<'a> {
        let mut x = Big5DistributionAnalysis { base: BaseCharDistributionAnalysis::new() };
        x.base.m_char_to_freq_order = Some(BIG5_CHAR_TO_FREQ_ORDER);
        x.base.m_table_size = BIG5_TABLE_SIZE;
        x.base.m_typical_distribution_ratio = BIG5_TYPICAL_DISTRIBUTION_RATIO;
        x
    }
}

impl<'a> CharDistributionAnalysis for Big5DistributionAnalysis<'a> {
    fn reset(&mut self) {
        self.base.reset();
    }
    fn get_confidence(&self) -> f32 {
        self.base.get_confidence()
    }
    fn got_enough_data(&self) -> bool {
        self.base.got_enough_data()
    }
    fn feed(&mut self, char: &[u8], char_len: usize) {
        let order: Option<usize>;
        if char_len == 2 {
            order = self.get_order(char);
        } else {
            order = None;
        }
        if order.is_some() {
            self.base.m_total_chars += 1;
            if order.unwrap() < self.base.m_table_size {
                let tmp = self.base.m_char_to_freq_order.unwrap();
                if 512 > tmp[order.unwrap() as usize] {
                    self.base.m_freq_chars += 1;
                }
            }
        }
    }
    fn get_order(&self, char: &[u8]) -> Option<usize> {
        if char.len() >= 2 {
            if char[0] >= 0xA4 {
                if char[1] >= 0xA1 {
                    let order: isize = 157 * (char[0] as isize - 0xA4) + char[1] as isize + 63 -
                        0xA1;
                    if order >= 0 {
                        Some(order as usize)
                    } else {
                        None
                    }
                } else {
                    let order: isize = 157 * (char[0] as isize - 0xA4) + char[1] as isize - 0x40;
                    if order >= 0 {
                        Some(order as usize)
                    } else {
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct SJISDistributionAnalysis<'a> {
    base: BaseCharDistributionAnalysis<'a>,
}

impl<'a> SJISDistributionAnalysis<'a> {
    pub fn new() -> SJISDistributionAnalysis<'a> {
        let mut x = SJISDistributionAnalysis { base: BaseCharDistributionAnalysis::new() };
        x.base.m_char_to_freq_order = Some(JIS_CHAR_TO_FREQ_ORDER);
        x.base.m_table_size = JIS_TABLE_SIZE;
        x.base.m_typical_distribution_ratio = JIS_TYPICAL_DISTRIBUTION_RATIO;
        x
    }
}

impl<'a> CharDistributionAnalysis for SJISDistributionAnalysis<'a> {
    fn reset(&mut self) {
        self.base.reset();
    }
    fn get_confidence(&self) -> f32 {
        self.base.get_confidence()
    }
    fn got_enough_data(&self) -> bool {
        self.base.got_enough_data()
    }
    fn feed(&mut self, char: &[u8], char_len: usize) {
        let order: Option<usize>;
        if char_len == 2 {
            order = self.get_order(char);
        } else {
            order = None;
        }
        if order.is_some() {
            self.base.m_total_chars += 1;
            if order.unwrap() < self.base.m_table_size {
                let tmp = self.base.m_char_to_freq_order.unwrap();
                if 512 > tmp[order.unwrap() as usize] {
                    self.base.m_freq_chars += 1;
                }
            }
        }
    }
    fn get_order(&self, char: &[u8]) -> Option<usize> {
        if char.len() >= 2 {
            let mut order: isize;
            if (char[0] >= 0x81) && (char[0] <= 0x9F) {
                order = 188 * (char[0] as isize - 0x81);
            } else if (char[0] >= 0xE0) && (char[0] <= 0xEF) {
                order = 188 * (char[0] as isize - 0xE0 + 31);
            } else {
                return None;
            }
            order = order + char[1] as isize - 0x40;
            if char[1] > 0x7F {
                return None;
            } else {
                if order >= 0 {
                    return Some(order as usize);
                } else {
                    return None;
                }
            }
        } else {
            None
        }
    }
}

pub struct EUCJPDistributionAnalysis<'a> {
    base: BaseCharDistributionAnalysis<'a>,
}

impl<'a> EUCJPDistributionAnalysis<'a> {
    pub fn new() -> EUCJPDistributionAnalysis<'a> {
        let mut x = EUCJPDistributionAnalysis { base: BaseCharDistributionAnalysis::new() };
        x.base.m_char_to_freq_order = Some(JIS_CHAR_TO_FREQ_ORDER);
        x.base.m_table_size = JIS_TABLE_SIZE;
        x.base.m_typical_distribution_ratio = JIS_TYPICAL_DISTRIBUTION_RATIO;
        x
    }
}

impl<'a> CharDistributionAnalysis for EUCJPDistributionAnalysis<'a> {
    fn reset(&mut self) {
        self.base.reset();
    }
    fn get_confidence(&self) -> f32 {
        self.base.get_confidence()
    }
    fn got_enough_data(&self) -> bool {
        self.base.got_enough_data()
    }
    fn feed(&mut self, char: &[u8], char_len: usize) {
        let order: Option<usize>;
        if char_len == 2 {
            order = self.get_order(char);
        } else {
            order = None;
        }
        if order.is_some() {
            self.base.m_total_chars += 1;
            if order.unwrap() < self.base.m_table_size {
                let tmp = self.base.m_char_to_freq_order.unwrap();
                if 512 > tmp[order.unwrap() as usize] {
                    self.base.m_freq_chars += 1;
                }
            }
        }
    }
    fn get_order(&self, char: &[u8]) -> Option<usize> {
        if char.len() >= 2 {
            if char[0] >= 0xA0 {
                let order:isize = 94 * (char[0] as isize - 0xA1) + char[1] as isize - 0xA1;
                if order >= 0 {
                    Some(order as usize)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
