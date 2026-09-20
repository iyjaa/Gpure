#[derive(Debug, thiserror::Error)]
pub enum GpureError {
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Build failed: {0}")]
    Build(String),
    #[error("Repository operation failed: {0}")]
    Repo(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = GpureError::Config("invalid path".to_string());
        assert_eq!(format!("{}", err), "Configuration error: invalid path");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
        let gpure_err: GpureError = io_err.into();
        
        match gpure_err {
            GpureError::Io(e) => assert_eq!(e.kind(), std::io::ErrorKind::NotFound),
            _ => panic!("Expected Io variant"),
        }
    }
}
