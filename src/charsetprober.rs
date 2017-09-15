use super::enums::ProbingState;

pub trait CharsetProber {
    fn reset(&mut self);
    fn feed(&mut self, byte_str: &[u8]) -> &ProbingState;
    fn get_charset(&self) -> String;
    fn get_confidence(&self) -> f32;
    fn get_language(&self) -> String;
    fn get_state(&self) -> &ProbingState;
}

pub fn filter_high_byte_only(byte_str: &[u8]) -> Vec<u8> {
    let mut filtered: Vec<u8> = Vec::new();
    for curr in 0..byte_str.len() {
        let buf_char = byte_str[curr];
        if buf_char < 0x80 {
            filtered.push(0x20); // space:0x20
        } else {
            filtered.push(buf_char);
        }
    }
    filtered
}

pub fn filter_international_words(byte_str: &[u8]) -> Vec<u8> {
    let mut filtered: Vec<u8> = Vec::new();
    let mut meet_msb: bool = false;
    let mut prev: usize = 0;
    for curr in 0..byte_str.len() {
        let buf_char = byte_str[curr];
        if buf_char >= 0x80 {
            meet_msb = true;
        } else if (buf_char < 65) || ((buf_char > 90) && (buf_char < 97)) || (buf_char > 122) {
            if meet_msb && (curr > prev) {
                while prev < curr {
                    filtered.push(byte_str[prev]);
                    prev += 1;
                }
                filtered.push(0x20); // space:0x20
                meet_msb = false;
            }
            prev = curr + 1;
        }
    }
    if meet_msb && (byte_str.len() > prev) {
        while prev < byte_str.len() {
            filtered.push(byte_str[prev]);
            prev += 1;
        }
    }
    filtered
}

pub fn filter_with_english_letters(byte_str: &[u8]) -> Vec<u8> {
    let mut filtered: Vec<u8> = Vec::new();
    let mut intag: bool = false;
    let mut prev: usize = 0;
    for curr in 0..byte_str.len() {
        let buf_char = byte_str[curr];
        if buf_char == 62 {
            // ">"
            intag = false;
        } else if buf_char == 60 {
            // "<"
            intag = true;
        }
        if (buf_char < 0x80) &&
            ((buf_char < 65) || ((buf_char > 90) && (buf_char < 97)) || (buf_char > 122))
        {
            if (curr > prev) && (!intag) {
                while prev < curr {
                    filtered.push(byte_str[prev]);
                    prev += 1;
                }
                filtered.push(0x20); // space:0x20
            }
            prev = curr + 1;
        }
    }
    if !intag {
        while prev < byte_str.len() {
            filtered.push(byte_str[prev]);
            prev += 1;
        }
    }
    filtered
}
