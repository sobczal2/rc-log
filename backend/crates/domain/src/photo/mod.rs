pub mod id;
pub mod path;
pub mod size;
pub mod resolver;
pub mod transaction;
pub mod service;

pub use id::PhotoId;

use crate::photo::path::Path;
use crate::photo::size::Size;

#[derive(Debug, Clone)]
pub struct Photo {
    pub id: PhotoId,
    pub small_path: Path,
    pub medium_path: Option<Path>,
    pub large_path: Option<Path>,
}

impl Photo {
    pub fn new(
        id: PhotoId,
        small_path: Path,
        medium_path: Option<Path>,
        large_path: Option<Path>,
    ) -> Self {
        Self { id, small_path, medium_path, large_path }
    }

    pub fn resolve_path(&self, size: Size) -> &Path {
        match size {
            Size::Large => {
                self.large_path.as_ref().or(self.medium_path.as_ref()).unwrap_or(&self.small_path)
            }
            Size::Medium => self.medium_path.as_ref().unwrap_or(&self.small_path),
            Size::Small => &self.small_path,
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::photo::path::Path;
    use crate::photo::size::Size;

    use super::{Photo, PhotoId};

    fn path(s: &str) -> Path {
        Path::new(s.to_string()).unwrap()
    }

    fn make_photo(small: &str, medium: Option<&str>, large: Option<&str>) -> Photo {
        Photo::new(PhotoId::new(Uuid::nil()), path(small), medium.map(path), large.map(path))
    }

    #[test]
    fn small_returns_small_path() {
        let p = make_photo("s.jpg", Some("m.jpg"), Some("l.jpg"));
        assert_eq!(p.resolve_path(Size::Small).as_str(), "s.jpg");
    }

    #[test]
    fn medium_returns_medium_when_present() {
        let p = make_photo("s.jpg", Some("m.jpg"), None);
        assert_eq!(p.resolve_path(Size::Medium).as_str(), "m.jpg");
    }

    #[test]
    fn medium_falls_back_to_small() {
        let p = make_photo("s.jpg", None, None);
        assert_eq!(p.resolve_path(Size::Medium).as_str(), "s.jpg");
    }

    #[test]
    fn large_returns_large_when_present() {
        let p = make_photo("s.jpg", Some("m.jpg"), Some("l.jpg"));
        assert_eq!(p.resolve_path(Size::Large).as_str(), "l.jpg");
    }

    #[test]
    fn large_falls_back_to_medium() {
        let p = make_photo("s.jpg", Some("m.jpg"), None);
        assert_eq!(p.resolve_path(Size::Large).as_str(), "m.jpg");
    }

    #[test]
    fn large_falls_back_to_small_when_only_small() {
        let p = make_photo("s.jpg", None, None);
        assert_eq!(p.resolve_path(Size::Large).as_str(), "s.jpg");
    }
}
