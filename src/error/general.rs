use std::fmt;

#[derive(Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum GeneralError {
    Message(String),
}

impl fmt::Display for GeneralError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeneralError::Message(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for GeneralError {}
