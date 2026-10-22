use crate::error::{INVALID_FILE_PATH, VerificationError};
use std::ffi::OsStr;
use std::path::Path;

pub struct FilePathVerifier;

impl FilePathVerifier {
    pub fn new() -> Self {
        Self
    }

    pub fn verify(&self, path: &str) -> Result<(), VerificationError> {
        let file_path = Path::new(path);
        if file_path.extension() != Some(OsStr::new("rs")) {
            return Err(VerificationError {
                exit_code: INVALID_FILE_PATH,
                errors: vec![format!("File path must have an .rs extension: {}", path)],
            });
        }
        if file_path.file_name() == Some(OsStr::new("build.rs")) {
            return Err(VerificationError {
                exit_code: INVALID_FILE_PATH,
                errors: vec![format!("File path must not be build.rs: {}", path)],
            });
        }
        Ok(())
    }
}

impl Default for FilePathVerifier {
    fn default() -> Self {
        FilePathVerifier::new()
    }
}

#[cfg(test)]
mod tests {
    use super::FilePathVerifier;
    use crate::error::INVALID_FILE_PATH;

    #[test]
    fn accepts_rs_extension() {
        let verifier = FilePathVerifier::new();
        assert!(verifier.verify("algorithm.rs").is_ok());
        assert!(verifier.verify("algorithms/example.rs").is_ok());
    }

    #[test]
    fn rejects_non_rs_extension() {
        let verifier = FilePathVerifier::new();
        for path in [
            "algorithm.txt",
            "algorithm",
            "algorithm.rs.txt",
            "algorithm.RS",
        ] {
            let error = verifier.verify(path).unwrap_err();
            assert_eq!(error.exit_code, INVALID_FILE_PATH);
        }
    }

    #[test]
    fn rejects_build_rs() {
        let verifier = FilePathVerifier::new();
        for path in ["build.rs", "algorithms/build.rs", "./build.rs"] {
            let error = verifier.verify(path).unwrap_err();
            assert_eq!(error.exit_code, INVALID_FILE_PATH);
        }
    }
}
