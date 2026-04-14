use std::fmt;

pub type AnankeResult<T> = Result<T, AnankeError>;

#[derive(Debug)]
pub enum AnankeError {
    Generic(String),
    Io(std::io::Error),
}

impl fmt::Display for AnankeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnankeError::Generic(msg) => write!(f, "{}", msg),
            AnankeError::Io(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for AnankeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AnankeError::Generic(_) => None,
            AnankeError::Io(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for AnankeError {
    fn from(err: std::io::Error) -> Self {
        AnankeError::Io(err)
    }
}
