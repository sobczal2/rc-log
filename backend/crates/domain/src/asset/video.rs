use uuid::Uuid;

use super::name::AssetName;
use super::path::AssetPath;
use super::size::AssetSize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VideoId(Uuid);

impl VideoId {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for VideoId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<VideoId> for Uuid {
    fn from(id: VideoId) -> Uuid {
        id.0
    }
}

#[derive(Debug, Clone)]
pub struct Video {
    pub id: VideoId,
    pub name: AssetName,
    pub small_path: AssetPath,
    pub medium_path: Option<AssetPath>,
    pub large_path: Option<AssetPath>,
}

impl Video {
    pub fn new(
        id: VideoId,
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

    use super::{Video, VideoId};

    fn path(s: &str) -> AssetPath {
        AssetPath::new(s.to_string()).unwrap()
    }

    fn make_video(small: &str, medium: Option<&str>, large: Option<&str>) -> Video {
        Video::new(
            VideoId::new(Uuid::nil()),
            AssetName::new("test".to_string()).unwrap(),
            path(small),
            medium.map(|s| path(s)),
            large.map(|s| path(s)),
        )
    }

    #[test]
    fn small_returns_small_path() {
        let v = make_video("s.mp4", Some("m.mp4"), Some("l.mp4"));
        assert_eq!(v.resolve_path(AssetSize::Small).as_str(), "s.mp4");
    }

    #[test]
    fn medium_returns_medium_when_present() {
        let v = make_video("s.mp4", Some("m.mp4"), None);
        assert_eq!(v.resolve_path(AssetSize::Medium).as_str(), "m.mp4");
    }

    #[test]
    fn medium_falls_back_to_small() {
        let v = make_video("s.mp4", None, None);
        assert_eq!(v.resolve_path(AssetSize::Medium).as_str(), "s.mp4");
    }

    #[test]
    fn large_returns_large_when_present() {
        let v = make_video("s.mp4", Some("m.mp4"), Some("l.mp4"));
        assert_eq!(v.resolve_path(AssetSize::Large).as_str(), "l.mp4");
    }

    #[test]
    fn large_falls_back_to_medium() {
        let v = make_video("s.mp4", Some("m.mp4"), None);
        assert_eq!(v.resolve_path(AssetSize::Large).as_str(), "m.mp4");
    }

    #[test]
    fn large_falls_back_to_small_when_only_small() {
        let v = make_video("s.mp4", None, None);
        assert_eq!(v.resolve_path(AssetSize::Large).as_str(), "s.mp4");
    }
}
