use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssetPath(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssetPathError {
    Empty,
}

impl fmt::Display for AssetPathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssetPathError::Empty => write!(f, "asset path must not be empty"),
        }
    }
}

impl std::error::Error for AssetPathError {}

impl AssetPath {
    pub fn new(value: String) -> Result<Self, AssetPathError> {
        if value.trim().is_empty() {
            return Err(AssetPathError::Empty);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{AssetPath, AssetPathError};

    #[test]
    fn valid_path() {
        let p = AssetPath::new("/assets/video/small.mp4".to_string()).unwrap();
        assert_eq!(p.as_str(), "/assets/video/small.mp4");
    }

    #[test]
    fn empty_is_err() {
        assert_eq!(AssetPath::new("".to_string()), Err(AssetPathError::Empty));
    }

    #[test]
    fn whitespace_only_is_err() {
        assert_eq!(AssetPath::new("   ".to_string()), Err(AssetPathError::Empty));
    }
}
