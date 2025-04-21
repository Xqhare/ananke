use anansi::AnansiError;


#[derive(Debug)]
pub struct AnankeError {
    pub title: String,
    pub message: String,
    pub context: Option<String>,
}

impl AnankeError {
    pub fn new<S: Into<String>>(title: S, message: S, context: Option<S>) -> AnankeError {
        AnankeError {
            title: title.into(),
            message: message.into(),
            context: context.map(|s| s.into()),
        }
    }
}

impl From<AnansiError> for AnankeError {
    fn from(e: AnansiError) -> Self {
        AnankeError {
            title: e.title,
            message: e.message,
            context: None,
        }
    }
}

// Display trait
impl std::fmt::Display for AnankeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{} - {}", self.title, self.message, self.context.clone().unwrap_or("".to_string()))
    }
}
