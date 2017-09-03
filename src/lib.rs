mod enums;
mod codingstatemachine;
mod mbcssm;
mod escsm;
mod big5freq;
mod euckrfreq;
mod euctwfreq;
mod gb2312freq;
mod jisfreq;
mod jpcntx;
mod chardistribution;
mod charsetprober;
mod mbcharsetprober;
mod big5prober;
mod cp949prober;
mod eucjpprober;
mod euckrprober;
mod euctwprober;
mod gb2312prober;
mod sjisprober;
mod utf8prober;
mod escprober;
mod sbcharsetprober;
mod langbulgarianmodel;
mod langcyrillicmodel;
mod langgreekmodel;
mod langhebrewmodel;
mod langthaimodel;
mod langturkishmodel;
mod hebrewprober;
mod latin1prober;

#[allow(dead_code)]
pub struct UniversalDetector {
    m_input_state: enums::InputState,
    m_done: bool,
    m_start: bool,
    m_got_data: bool,
    m_last_char: u8,
    m_detected_charset: String,
    m_detected_confidence: f32,
    m_detected_language: String,
    m_esc_charset_prober: Option<Box<charsetprober::CharsetProber>>,
    m_charset_probers: Vec<Box<charsetprober::CharsetProber>>,
}

impl UniversalDetector {
    pub fn new() -> UniversalDetector {
        UniversalDetector {
            m_input_state: enums::InputState::PureAscii,
            m_done: false,
            m_start: true,
            m_got_data: false,
            m_last_char: 0,
            m_detected_charset: String::new(),
            m_detected_confidence: 0.0,
            m_detected_language: String::new(),
            m_esc_charset_prober: None,
            m_charset_probers: Vec::new(),
        }
    }
    pub fn reset(&mut self) {
        self.m_input_state = enums::InputState::PureAscii;
        self.m_done = false;
        self.m_start = true;
        self.m_got_data = false;
        self.m_last_char = 0;
        self.m_detected_charset = String::new();
        self.m_detected_confidence = 0.0;
        self.m_detected_language = String::new();
        match self.m_esc_charset_prober {
            Some(ref mut prober) => {
                prober.reset();
            }
            _ => {}
        }
        self.m_charset_probers.clear();
    }
    pub fn feed(&mut self, byte_str: &Vec<u8>) {
        if self.m_done {
            return;
        }
        if byte_str.len() <= 0 {
            return;
        }
        if !self.m_got_data {
            if byte_str.len() >= 2 {
                match byte_str[0] {
                    0x00 => {
                        if (byte_str.len() >= 4) && (byte_str[1] == 0x00) {
                            if (byte_str[2] == 0xfe) && (byte_str[3] == 0xff) {
                                self.m_detected_charset = "UTF-32BE".to_string();
                            } else if (byte_str[2] == 0xff) && (byte_str[3] == 0xfe) {
                                self.m_detected_charset = "X-ISO-100646-UCS-4-2143".to_string();
                            }
                            self.m_detected_confidence = 1.0;
                            self.m_detected_language = "".to_string();
                        }
                    }
                    0xef => {
                        if (byte_str.len() > 2) && (byte_str[1] == 0xbb) && (byte_str[2] == 0xbf) {
                            self.m_detected_charset = "UTF-8".to_string();
                            self.m_detected_confidence = 1.0;
                            self.m_detected_language = "".to_string();
                        }
                    }
                    0xfe => {
                        if byte_str[1] == 0xff {
                            if (byte_str.len() >= 4) && (byte_str[2] == 0x00) &&
                                (byte_str[3] == 0x00)
                            {
                                self.m_detected_charset = "X-ISO-10646-UCS-4-3412".to_string();
                            } else {
                                self.m_detected_charset = "UTF-16BE".to_string();
                            }
                            self.m_detected_confidence = 1.0;
                            self.m_detected_language = "".to_string();
                        }
                    }
                    0xff => {
                        if byte_str[1] == 0xfe {
                            if (byte_str.len() >= 4) && (byte_str[2] == 0x00) &&
                                (byte_str[3] == 0x00)
                            {
                                self.m_detected_charset = "UTF-32LE".to_string();
                            } else {
                                self.m_detected_charset = "UTF-16LE".to_string();
                            }
                            self.m_detected_confidence = 1.0;
                            self.m_detected_language = "".to_string();
                        }
                    }
                    _ => {}
                }
            }
            self.m_got_data = true;
            if self.m_detected_charset != "" {
                self.m_done = true;
                return;
            }
        }
        for &ch in byte_str {
            if (ch & 0x80 != 0) && (ch != 0xa0) {
                match self.m_input_state {
                    enums::InputState::Highbyte => {}
                    _ => {
                        self.m_input_state = enums::InputState::Highbyte;
                    }
                }
            } else {
                match self.m_input_state {
                    enums::InputState::PureAscii => {
                        if ch == 0x1B {
                            self.m_input_state = enums::InputState::EscAscii;
                        }
                    }
                    _ => {}
                }
            }
        }
        self.m_last_char = byte_str[byte_str.len() - 1];

        match self.m_input_state {
            enums::InputState::EscAscii => {
                if self.m_esc_charset_prober.is_none() {
                    self.m_esc_charset_prober = Some(Box::new(escprober::EscCharsetProber::new()));
                }
                let prober = self.m_esc_charset_prober.as_mut().unwrap();
                if *prober.feed(byte_str) == enums::ProbingState::FoundIt {
                    self.m_detected_charset = prober.get_charset();
                    self.m_detected_confidence = prober.get_confidence();
                    self.m_detected_language = prober.get_language();
                    self.m_done = true;
                }
            }
            enums::InputState::Highbyte => {
                if self.m_charset_probers.is_empty() {
                    // MultiByte
                    self.m_charset_probers.push(Box::new(
                        utf8prober::UTF8Prober::new(),
                    ));
                    self.m_charset_probers.push(Box::new(
                        sjisprober::SJISProber::new(),
                    ));
                    self.m_charset_probers.push(Box::new(
                        eucjpprober::EUCJPProber::new(),
                    ));
                    self.m_charset_probers.push(Box::new(
                        gb2312prober::GB2312Prober::new(),
                    ));
                    self.m_charset_probers.push(Box::new(
                        euckrprober::EUCKRProber::new(),
                    ));
                    self.m_charset_probers.push(Box::new(
                        cp949prober::CP949Prober::new(),
                    ));
                    self.m_charset_probers.push(Box::new(
                        big5prober::Big5Prober::new(),
                    ));
                    self.m_charset_probers.push(Box::new(
                        euctwprober::EUCTWProber::new(),
                    ));
                    // SingleByte
                    self.m_charset_probers.push(Box::new(
                        sbcharsetprober::SingleByteCharsetProber::new(
                            &langcyrillicmodel::Win1251CyrillicModel,
                            false,
                        ),
                    ));
                    self.m_charset_probers.push(Box::new(
                        sbcharsetprober::SingleByteCharsetProber::new(
                            &langcyrillicmodel::Koi8rModel,
                            false,
                        ),
                    ));
                    self.m_charset_probers.push(Box::new(
                        sbcharsetprober::SingleByteCharsetProber::new(
                            &langcyrillicmodel::Latin5CyrillicModel,
                            false,
                        ),
                    ));
                    self.m_charset_probers.push(Box::new(
                        sbcharsetprober::SingleByteCharsetProber::new(
                            &langcyrillicmodel::MacCyrillicModel,
                            false,
                        ),
                    ));
                    self.m_charset_probers.push(Box::new(
                        sbcharsetprober::SingleByteCharsetProber::new(
                            &langcyrillicmodel::Ibm866Model,
                            false,
                        ),
                    ));
                    self.m_charset_probers.push(Box::new(
                        sbcharsetprober::SingleByteCharsetProber::new(
                            &langcyrillicmodel::Ibm855Model,
                            false,
                        ),
                    ));
                    self.m_charset_probers.push(Box::new(
                        sbcharsetprober::SingleByteCharsetProber::new(
                            &langgreekmodel::Latin7GreekModel,
                            false,
                        ),
                    ));
                    self.m_charset_probers.push(Box::new(
                        sbcharsetprober::SingleByteCharsetProber::new(
                            &langgreekmodel::Win1253GreekModel,
                            false,
                        ),
                    ));
                    self.m_charset_probers.push(Box::new(
                        sbcharsetprober::SingleByteCharsetProber::new(
                            &langbulgarianmodel::Latin5BulgarianModel,
                            false,
                        ),
                    ));
                    self.m_charset_probers.push(Box::new(
                        sbcharsetprober::SingleByteCharsetProber::new(
                            &langbulgarianmodel::Win1251BulgarianModel,
                            false,
                        ),
                    ));
                    self.m_charset_probers.push(Box::new(
                        sbcharsetprober::SingleByteCharsetProber::new(
                            &langthaimodel::TIS620ThaiModel,
                            false,
                        ),
                    ));
                    self.m_charset_probers.push(Box::new(
                        sbcharsetprober::SingleByteCharsetProber::new(
                            &langturkishmodel::Latin5TurkishModel,
                            false,
                        ),
                    ));
                    self.m_charset_probers.push(
                        Box::new(hebrewprober::HebrewProber::new()),
                    );
                    self.m_charset_probers.push(Box::new(
                        latin1prober::Latin1Prober::new(),
                    ));
                }
                for x in &mut self.m_charset_probers {
                    if *x.feed(byte_str) == enums::ProbingState::FoundIt {
                        self.m_detected_charset = x.get_charset();
                        self.m_detected_confidence = x.get_confidence();
                        self.m_detected_language = x.get_language();
                        self.m_done = true;
                        break;
                    }
                }
            }
            _ => {}
        }
    }
    pub fn close(&mut self) -> (String, f32, String) {
        if self.m_done {
        } else {
            if self.m_got_data {
                match self.m_input_state {
                    enums::InputState::PureAscii => {
                        self.m_detected_charset = "ascii".to_string();
                        self.m_detected_confidence = 1.0;
                        self.m_detected_language = "".to_string();
                    }
                    enums::InputState::Highbyte => {
                        let mut maxidx: usize = 0;
                        let mut maxconfidence: f32 = 0.0;
                        for i in 0..self.m_charset_probers.len() {
                            let tmp = self.m_charset_probers[i].get_confidence();
                            if tmp > maxconfidence {
                                maxconfidence = tmp;
                                maxidx = i;
                            }
                        }
                        if maxconfidence > 0.2 {
                            self.m_detected_charset = self.m_charset_probers[maxidx].get_charset();
                            self.m_detected_confidence = self.m_charset_probers[maxidx]
                                .get_confidence();
                            self.m_detected_language = self.m_charset_probers[maxidx]
                                .get_language();
                        }
                    }
                    _ => unreachable!(),
                }
            } else {
            }
        }
        (
            self.m_detected_charset.clone(),
            self.m_detected_confidence,
            self.m_detected_language.clone(),
        )
    }
}

/// detect charset for given buffer
pub fn detect(byte_str: &Vec<u8>) -> (String, f32, String) {
    let mut detector = UniversalDetector::new();
    detector.feed(byte_str);
    detector.close()
}

/// translate charset name for encoding
pub fn charset2encoding(enc:&String) -> Option<&str> {
    match enc.as_str() {
        "CP932" => Some("windows-31j"),
        "CP949" => Some("windows-949"),
        "MacCyrillic" => Some("x-mac-cyrillic"),
        _ => Some(enc.as_str()),
    }
}
