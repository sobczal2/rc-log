use std::sync::Arc;

use moka::future::Cache;
use rc_log_domain::asset::name::AssetName;
use rc_log_domain::asset::path::AssetPath;
use rc_log_domain::asset::photo::{Photo, PhotoId};
use rc_log_domain::asset::photo_resolver::PhotoResolver;
use rc_log_domain::shared::transaction::TransactionError;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
struct PhotoRow {
    id: Uuid,
    name: String,
    small_path: String,
    medium_path: Option<String>,
    large_path: Option<String>,
}

impl PhotoRow {
    fn try_into_photo(self) -> Result<Photo, TransactionError> {
        let name =
            AssetName::new(self.name).map_err(|e| TransactionError::InvalidData(e.to_string()))?;
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
        Ok(Photo::new(PhotoId::new(self.id), name, small_path, medium_path, large_path))
    }
}

#[derive(Clone)]
pub struct SqlxPhotoResolver {
    pool: PgPool,
    cache: Cache<String, Arc<Photo>>,
}

impl SqlxPhotoResolver {
    pub fn new(pool: PgPool, capacity: u64) -> Self {
        let cache = Cache::new(capacity);
        Self { pool, cache }
    }
}

impl PhotoResolver for SqlxPhotoResolver {
    async fn get(&self, name: &AssetName) -> Result<Option<Photo>, TransactionError> {
        let key = name.as_str().to_string();

        if let Some(cached) = self.cache.get(&key).await {
            return Ok(Some((*cached).clone()));
        }

        let row: Option<PhotoRow> = sqlx::query_as(
            r#"
            SELECT id, name, small_path, medium_path, large_path
            FROM asset.photo
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
                let photo = Arc::new(row.try_into_photo()?);
                let result = (*photo).clone();
                self.cache.insert(key, photo).await;
                Ok(Some(result))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::PhotoRow;

    #[test]
    fn try_into_photo_valid_all_sizes() {
        let row = PhotoRow {
            id: Uuid::nil(),
            name: "banner".to_string(),
            small_path: "s.jpg".to_string(),
            medium_path: Some("m.jpg".to_string()),
            large_path: Some("l.jpg".to_string()),
        };
        let photo = row.try_into_photo().unwrap();
        assert_eq!(photo.name.as_str(), "banner");
        assert_eq!(photo.small_path.as_str(), "s.jpg");
        assert_eq!(photo.medium_path.as_ref().unwrap().as_str(), "m.jpg");
        assert_eq!(photo.large_path.as_ref().unwrap().as_str(), "l.jpg");
    }

    #[test]
    fn try_into_photo_valid_small_only() {
        let row = PhotoRow {
            id: Uuid::nil(),
            name: "thumb".to_string(),
            small_path: "s.jpg".to_string(),
            medium_path: None,
            large_path: None,
        };
        let photo = row.try_into_photo().unwrap();
        assert!(photo.medium_path.is_none());
        assert!(photo.large_path.is_none());
    }

    #[test]
    fn try_into_photo_empty_name_is_err() {
        let row = PhotoRow {
            id: Uuid::nil(),
            name: "  ".to_string(),
            small_path: "s.jpg".to_string(),
            medium_path: None,
            large_path: None,
        };
        assert!(row.try_into_photo().is_err());
    }

    #[test]
    fn try_into_photo_empty_small_path_is_err() {
        let row = PhotoRow {
            id: Uuid::nil(),
            name: "thumb".to_string(),
            small_path: "  ".to_string(),
            medium_path: None,
            large_path: None,
        };
        assert!(row.try_into_photo().is_err());
    }

    #[test]
    fn try_into_photo_empty_optional_path_is_err() {
        let row = PhotoRow {
            id: Uuid::nil(),
            name: "thumb".to_string(),
            small_path: "s.jpg".to_string(),
            medium_path: Some("  ".to_string()),
            large_path: None,
        };
        assert!(row.try_into_photo().is_err());
    }
}
