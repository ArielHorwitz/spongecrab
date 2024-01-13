/// Error type for spongecrab
#[derive(Debug)]
pub struct Error(String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self(value)
    }
}

/// Result type for spongecrab
pub type Result<T> = std::result::Result<T, Error>;
