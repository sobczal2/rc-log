use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathError {
    Empty,
}

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathError::Empty => write!(f, "asset path must not be empty"),
        }
    }
}

impl std::error::Error for PathError {}

impl Path {
    pub fn new(value: String) -> Result<Self, PathError> {
        if value.trim().is_empty() {
            return Err(PathError::Empty);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{Path, PathError};

    #[test]
    fn valid_path() {
        let p = Path::new("/assets/video/small.mp4".to_string()).unwrap();
        assert_eq!(p.as_str(), "/assets/video/small.mp4");
    }

    #[test]
    fn empty_is_err() {
        assert_eq!(Path::new("".to_string()), Err(PathError::Empty));
    }

    #[test]
    fn whitespace_only_is_err() {
        assert_eq!(Path::new("   ".to_string()), Err(PathError::Empty));
    }
}
