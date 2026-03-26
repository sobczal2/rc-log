use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VideoPath(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VideoPathError {
    Empty,
}

impl fmt::Display for VideoPathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VideoPathError::Empty => write!(f, "video path must not be empty"),
        }
    }
}

impl std::error::Error for VideoPathError {}

impl VideoPath {
    pub fn new(value: String) -> Result<Self, VideoPathError> {
        if value.trim().is_empty() {
            return Err(VideoPathError::Empty);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
