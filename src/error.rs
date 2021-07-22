use std::fmt;

#[derive(Debug, PartialEq)]
pub enum IPError {
    InvalidVersion(u8),
    NotEnoughBytes(u8),
    TooManyBytes(u32),
}

impl fmt::Display for IPError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IPError::InvalidVersion(v) => {
                write!(f, "ip packet version is invalid, provided {}", v)
            }
            IPError::NotEnoughBytes(b) => {
                write!(f, "ip packet doesn't have enough bytes, {} provided", b)
            }
            IPError::TooManyBytes(b) => {
                write!(f, "ip packet has way too much data, {} provided", b)
            }
        }
    }
}
