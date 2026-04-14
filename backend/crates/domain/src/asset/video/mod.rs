pub mod id;
pub mod resolver;
pub mod transaction;

pub use id::VideoId;

use crate::asset::path::Path;
use crate::asset::size::Size;

#[derive(Debug, Clone)]
pub struct Video {
    pub id: VideoId,
    pub small_path: Path,
    pub medium_path: Option<Path>,
    pub large_path: Option<Path>,
}

impl Video {
    pub fn new(
        id: VideoId,
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

    use crate::asset::path::Path;
    use crate::asset::size::Size;

    use super::{Video, VideoId};

    fn path(s: &str) -> Path {
        Path::new(s.to_string()).unwrap()
    }

    fn make_video(small: &str, medium: Option<&str>, large: Option<&str>) -> Video {
        Video::new(
            VideoId::new(Uuid::nil()),
            path(small),
            medium.map(path),
            large.map(path),
        )
    }

    #[test]
    fn small_returns_small_path() {
        let v = make_video("s.mp4", Some("m.mp4"), Some("l.mp4"));
        assert_eq!(v.resolve_path(Size::Small).as_str(), "s.mp4");
    }

    #[test]
    fn medium_returns_medium_when_present() {
        let v = make_video("s.mp4", Some("m.mp4"), None);
        assert_eq!(v.resolve_path(Size::Medium).as_str(), "m.mp4");
    }

    #[test]
    fn medium_falls_back_to_small() {
        let v = make_video("s.mp4", None, None);
        assert_eq!(v.resolve_path(Size::Medium).as_str(), "s.mp4");
    }

    #[test]
    fn large_returns_large_when_present() {
        let v = make_video("s.mp4", Some("m.mp4"), Some("l.mp4"));
        assert_eq!(v.resolve_path(Size::Large).as_str(), "l.mp4");
    }

    #[test]
    fn large_falls_back_to_medium() {
        let v = make_video("s.mp4", Some("m.mp4"), None);
        assert_eq!(v.resolve_path(Size::Large).as_str(), "m.mp4");
    }

    #[test]
    fn large_falls_back_to_small_when_only_small() {
        let v = make_video("s.mp4", None, None);
        assert_eq!(v.resolve_path(Size::Large).as_str(), "s.mp4");
    }
}
