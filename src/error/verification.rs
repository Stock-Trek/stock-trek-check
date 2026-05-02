#[derive(Debug)]
pub struct VerificationError {
    pub exit_code: u8,
    pub errors: Vec<String>,
}

pub const NO_SUCH_FILE: u8 = 1;
pub const PARSE_ERROR: u8 = 2;
pub const BLOCKED_SYNTAX_ERROR: u8 = 3;
