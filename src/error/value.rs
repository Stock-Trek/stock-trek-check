use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum ValueError {
    #[error("{name} not found for '{key}'")]
    NotFound { name: String, key: String },
    #[error("Value expects type '{expected}' but found type '{found}'")]
    IncorrectType { expected: String, found: String },
}
