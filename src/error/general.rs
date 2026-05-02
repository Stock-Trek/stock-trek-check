use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum GeneralError {
    #[error("{0}")]
    Message(String),
}
