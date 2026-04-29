use std::fmt;

use anansi::AnansiError;
use areia::error::AreiaError;
use brigid::error::BrigidError;
use talos::TalosError;

pub type AnankeResult<T> = Result<T, AnankeError>;

#[derive(Debug)]
pub enum AnankeError {
    Startup(String),
    Brigid(BrigidError),
    Anansi(AnansiError),
    Areia(AreiaError),
    Talos(TalosError),
    Io(std::io::Error),
}

impl fmt::Display for AnankeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnankeError::Talos(err) => write!(f, "{}", err),
            AnankeError::Io(err) => write!(f, "{}", err),
            AnankeError::Anansi(err) => write!(f, "{}", err),
            AnankeError::Areia(err) => write!(f, "{}", err),
            AnankeError::Brigid(err) => write!(f, "{}", err),
            AnankeError::Startup(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for AnankeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AnankeError::Talos(_)
            | AnankeError::Anansi(_)
            | AnankeError::Startup(_)
            | AnankeError::Areia(_)
            | AnankeError::Brigid(_) => None,
            AnankeError::Io(err) => Some(err),
        }
    }
}

impl From<TalosError> for AnankeError {
    fn from(err: TalosError) -> Self {
        AnankeError::Talos(err)
    }
}

impl From<BrigidError> for AnankeError {
    fn from(err: BrigidError) -> Self {
        AnankeError::Brigid(err)
    }
}

impl From<AnansiError> for AnankeError {
    fn from(err: AnansiError) -> Self {
        AnankeError::Anansi(err)
    }
}

impl From<AreiaError> for AnankeError {
    fn from(err: AreiaError) -> Self {
        AnankeError::Areia(err)
    }
}

impl From<std::io::Error> for AnankeError {
    fn from(err: std::io::Error) -> Self {
        AnankeError::Io(err)
    }
}
