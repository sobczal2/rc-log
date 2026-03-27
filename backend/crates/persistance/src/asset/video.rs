use std::sync::Arc;

use moka::future::Cache;
use rc_log_domain::asset::name::AssetName;
use rc_log_domain::asset::path::AssetPath;
use rc_log_domain::asset::video::Video;
use rc_log_domain::asset::video_resolver::VideoResolver;
use rc_log_domain::shared::transaction::TransactionError;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
struct VideoRow {
    id: Uuid,
    name: String,
    small_path: String,
    medium_path: Option<String>,
    large_path: Option<String>,
}

impl VideoRow {
    fn try_into_video(self) -> Result<Video, TransactionError> {
        let name = AssetName::new(self.name)
            .map_err(|e| TransactionError::InvalidData(e.to_string()))?;
        let small_path = AssetPath::new(self.small_path)
            .map_err(|e| TransactionError::InvalidData(e.to_string()))?;
        let medium_path = self
            .medium_path
            .map(AssetPath::new)
            .transpose()
            .map_err(|e| TransactionError::InvalidData(e.to_string()))?;
        let large_path = self
            .large_path
            .map(AssetPath::new)
            .transpose()
            .map_err(|e| TransactionError::InvalidData(e.to_string()))?;
        Ok(Video::new(self.id, name, small_path, medium_path, large_path))
    }
}

#[derive(Clone)]
pub struct SqlxVideoResolver {
    pool: PgPool,
    cache: Cache<String, Arc<Video>>,
}

impl SqlxVideoResolver {
    pub fn new(pool: PgPool, capacity: u64) -> Self {
        let cache = Cache::new(capacity);
        Self { pool, cache }
    }
}

impl VideoResolver for SqlxVideoResolver {
    async fn get(&self, name: &AssetName) -> Result<Option<Video>, TransactionError> {
        let key = name.as_str().to_string();

        if let Some(cached) = self.cache.get(&key).await {
            return Ok(Some((*cached).clone()));
        }

        let row: Option<VideoRow> = sqlx::query_as(
            r#"
            SELECT id, name, small_path, medium_path, large_path
            FROM asset.video
            WHERE name = $1
            "#,
        )
        .bind(name.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        match row {
            None => Ok(None),
            Some(row) => {
                let video = Arc::new(row.try_into_video()?);
                let result = (*video).clone();
                self.cache.insert(key, video).await;
                Ok(Some(result))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::VideoRow;

    #[test]
    fn try_into_video_valid_all_sizes() {
        let row = VideoRow {
            id: Uuid::nil(),
            name: "hero".to_string(),
            small_path: "s.mp4".to_string(),
            medium_path: Some("m.mp4".to_string()),
            large_path: Some("l.mp4".to_string()),
        };
        let video = row.try_into_video().unwrap();
        assert_eq!(video.name.as_str(), "hero");
        assert_eq!(video.small_path.as_str(), "s.mp4");
        assert_eq!(video.medium_path.as_ref().unwrap().as_str(), "m.mp4");
        assert_eq!(video.large_path.as_ref().unwrap().as_str(), "l.mp4");
    }

    #[test]
    fn try_into_video_valid_small_only() {
        let row = VideoRow {
            id: Uuid::nil(),
            name: "clip".to_string(),
            small_path: "s.mp4".to_string(),
            medium_path: None,
            large_path: None,
        };
        let video = row.try_into_video().unwrap();
        assert!(video.medium_path.is_none());
        assert!(video.large_path.is_none());
    }

    #[test]
    fn try_into_video_empty_name_is_err() {
        let row = VideoRow {
            id: Uuid::nil(),
            name: "  ".to_string(),
            small_path: "s.mp4".to_string(),
            medium_path: None,
            large_path: None,
        };
        assert!(row.try_into_video().is_err());
    }

    #[test]
    fn try_into_video_empty_small_path_is_err() {
        let row = VideoRow {
            id: Uuid::nil(),
            name: "clip".to_string(),
            small_path: "  ".to_string(),
            medium_path: None,
            large_path: None,
        };
        assert!(row.try_into_video().is_err());
    }

    #[test]
    fn try_into_video_empty_optional_path_is_err() {
        let row = VideoRow {
            id: Uuid::nil(),
            name: "clip".to_string(),
            small_path: "s.mp4".to_string(),
            medium_path: Some("  ".to_string()),
            large_path: None,
        };
        assert!(row.try_into_video().is_err());
    }
}
