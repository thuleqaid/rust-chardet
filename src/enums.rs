#[derive(PartialEq, Debug)]
pub enum InputState {
    PureAscii,
    EscAscii,
    Highbyte,
}

#[derive(PartialEq, Debug)]
pub enum ProbingState {
    Detecting,
    FoundIt,
    NotMe,
}

#[allow(non_snake_case)]
pub mod MachineState {
    pub const START:u8 = 0;
    pub const ERROR:u8 = 1;
    pub const ITS_ME:u8 = 2;
}

#[allow(non_snake_case, dead_code)]
pub mod SequenceLikelihood {
    pub const NEGATIVE:usize = 0;
    pub const UNLIKELY:usize = 1;
    pub const LIKELY:usize = 2;
    pub const POSITIVE:usize = 3;
    pub const CATEGORIES:usize = 4;
}

#[allow(non_snake_case, dead_code)]
pub mod CharacterCategory {
    pub const UNDEFINED:u8 = 255;
    pub const LINE_BREAK:u8 = 254;
    pub const SYMBOL:u8 = 253;
    pub const DIGIT:u8 = 252;
    pub const CONTROL:u8 = 251;
}

