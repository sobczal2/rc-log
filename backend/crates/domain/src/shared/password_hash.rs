#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(inner: String) -> Self {
        Self(inner)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
