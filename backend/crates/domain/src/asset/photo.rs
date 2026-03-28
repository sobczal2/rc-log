use uuid::Uuid;

use super::name::AssetName;
use super::path::AssetPath;
use super::size::AssetSize;

#[derive(Debug, Clone)]
pub struct Photo {
    pub id: Uuid,
    pub name: AssetName,
    pub small_path: AssetPath,
    pub medium_path: Option<AssetPath>,
    pub large_path: Option<AssetPath>,
}

impl Photo {
    pub fn new(
        id: Uuid,
        name: AssetName,
        small_path: AssetPath,
        medium_path: Option<AssetPath>,
        large_path: Option<AssetPath>,
    ) -> Self {
        Self { id, name, small_path, medium_path, large_path }
    }

    /// Resolves a path for the requested size, falling back to the next smaller
    /// available size. `small_path` is always present and is the final fallback.
    pub fn resolve_path(&self, size: AssetSize) -> &AssetPath {
        match size {
            AssetSize::Large => {
                self.large_path.as_ref().or(self.medium_path.as_ref()).unwrap_or(&self.small_path)
            }
            AssetSize::Medium => self.medium_path.as_ref().unwrap_or(&self.small_path),
            AssetSize::Small => &self.small_path,
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::asset::name::AssetName;
    use crate::asset::path::AssetPath;
    use crate::asset::size::AssetSize;

    use super::Photo;

    fn path(s: &str) -> AssetPath {
        AssetPath::new(s.to_string()).unwrap()
    }

    fn make_photo(small: &str, medium: Option<&str>, large: Option<&str>) -> Photo {
        Photo::new(
            Uuid::nil(),
            AssetName::new("test".to_string()).unwrap(),
            path(small),
            medium.map(|s| path(s)),
            large.map(|s| path(s)),
        )
    }

    #[test]
    fn small_returns_small_path() {
        let p = make_photo("s.jpg", Some("m.jpg"), Some("l.jpg"));
        assert_eq!(p.resolve_path(AssetSize::Small).as_str(), "s.jpg");
    }

    #[test]
    fn medium_returns_medium_when_present() {
        let p = make_photo("s.jpg", Some("m.jpg"), None);
        assert_eq!(p.resolve_path(AssetSize::Medium).as_str(), "m.jpg");
    }

    #[test]
    fn medium_falls_back_to_small() {
        let p = make_photo("s.jpg", None, None);
        assert_eq!(p.resolve_path(AssetSize::Medium).as_str(), "s.jpg");
    }

    #[test]
    fn large_returns_large_when_present() {
        let p = make_photo("s.jpg", Some("m.jpg"), Some("l.jpg"));
        assert_eq!(p.resolve_path(AssetSize::Large).as_str(), "l.jpg");
    }

    #[test]
    fn large_falls_back_to_medium() {
        let p = make_photo("s.jpg", Some("m.jpg"), None);
        assert_eq!(p.resolve_path(AssetSize::Large).as_str(), "m.jpg");
    }

    #[test]
    fn large_falls_back_to_small_when_only_small() {
        let p = make_photo("s.jpg", None, None);
        assert_eq!(p.resolve_path(AssetSize::Large).as_str(), "s.jpg");
    }
}
