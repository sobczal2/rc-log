use rc_log_domain::asset::name::Name;
use rc_log_domain::asset::path::Path;
use rc_log_domain::asset::photo::{Photo, PhotoId};
use rc_log_domain::asset::photo::transaction::PhotoTransaction;
use rc_log_domain::shared::transaction::{Transaction, TransactionError};
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use sqlx::{PgPool, Postgres, Transaction as SqlxTransaction};
use uuid::Uuid;

// ─── Row ──────────────────────────────────────────────────────────────────────

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
            Name::new(self.name).map_err(|e| TransactionError::InvalidData(e.to_string()))?;
        let small_path = Path::new(self.small_path)
            .map_err(|e| TransactionError::InvalidData(e.to_string()))?;
        let medium_path = self
            .medium_path
            .map(Path::new)
            .transpose()
            .map_err(|e| TransactionError::InvalidData(e.to_string()))?;
        let large_path = self
            .large_path
            .map(Path::new)
            .transpose()
            .map_err(|e| TransactionError::InvalidData(e.to_string()))?;
        Ok(Photo::new(PhotoId::new(self.id), name, small_path, medium_path, large_path))
    }

    fn from_photo(photo: &Photo) -> Self {
        Self {
            id: Uuid::from(photo.id),
            name: photo.name.as_str().to_string(),
            small_path: photo.small_path.as_str().to_string(),
            medium_path: photo.medium_path.as_ref().map(|p| p.as_str().to_string()),
            large_path: photo.large_path.as_ref().map(|p| p.as_str().to_string()),
        }
    }
}

// ─── Transaction ──────────────────────────────────────────────────────────────

pub struct SqlxPhotoTransaction {
    tx: SqlxTransaction<'static, Postgres>,
}

impl Transaction<Photo> for SqlxPhotoTransaction {
    async fn save(&mut self, photo: &Photo) -> Result<(), TransactionError> {
        let row = PhotoRow::from_photo(photo);

        sqlx::query(
            r#"
            INSERT INTO asset.photo (id, name, small_path, medium_path, large_path)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (name) DO UPDATE SET
                small_path  = EXCLUDED.small_path,
                medium_path = EXCLUDED.medium_path,
                large_path  = EXCLUDED.large_path
            "#,
        )
        .bind(row.id)
        .bind(&row.name)
        .bind(&row.small_path)
        .bind(&row.medium_path)
        .bind(&row.large_path)
        .execute(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        Ok(())
    }

    async fn commit(self) -> Result<(), TransactionError> {
        self.tx.commit().await.map_err(|e| TransactionError::TransactionError(e.to_string()))
    }

    async fn rollback(self) -> Result<(), TransactionError> {
        self.tx.rollback().await.map_err(|e| TransactionError::TransactionError(e.to_string()))
    }
}

impl PhotoTransaction for SqlxPhotoTransaction {
    async fn get_by_name(&mut self, name: &Name) -> Result<Option<Photo>, TransactionError> {
        let row: Option<PhotoRow> = sqlx::query_as(
            r#"
            SELECT id, name, small_path, medium_path, large_path
            FROM asset.photo
            WHERE name = $1
            "#,
        )
        .bind(name.as_str())
        .fetch_optional(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        row.map(PhotoRow::try_into_photo).transpose()
    }

    async fn delete_by_name(&mut self, name: &Name) -> Result<(), TransactionError> {
        sqlx::query(
            r#"
            DELETE FROM asset.photo
            WHERE name = $1
            "#,
        )
        .bind(name.as_str())
        .execute(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        Ok(())
    }
}

// ─── Unit of Work ─────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct SqlxPhotoUnitOfWork {
    pool: PgPool,
}

impl SqlxPhotoUnitOfWork {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UnitOfWork<Photo> for SqlxPhotoUnitOfWork {
    type Transaction = SqlxPhotoTransaction;

    async fn begin(&mut self) -> Result<Self::Transaction, TransactionError> {
        let tx = self
            .pool
            .begin()
            .await
            .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        Ok(SqlxPhotoTransaction { tx })
    }
}

// ─── Unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::PhotoRow;

    #[test]
    fn try_into_photo_valid_all_sizes() {
        let row = PhotoRow {
            id: Uuid::nil(),
            name: "banner".to_string(),
            small_path: "photos/banner_small.webp".to_string(),
            medium_path: Some("photos/banner_medium.webp".to_string()),
            large_path: Some("photos/banner_large.webp".to_string()),
        };
        let photo = row.try_into_photo().unwrap();
        assert_eq!(photo.name.as_str(), "banner");
        assert_eq!(photo.small_path.as_str(), "photos/banner_small.webp");
        assert!(photo.medium_path.is_some());
        assert!(photo.large_path.is_some());
    }

    #[test]
    fn try_into_photo_valid_small_only() {
        let row = PhotoRow {
            id: Uuid::nil(),
            name: "thumb".to_string(),
            small_path: "photos/thumb_small.webp".to_string(),
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
            name: "".to_string(),
            small_path: "photos/x_small.webp".to_string(),
            medium_path: None,
            large_path: None,
        };
        assert!(row.try_into_photo().is_err());
    }

    #[test]
    fn try_into_photo_empty_small_path_is_err() {
        let row = PhotoRow {
            id: Uuid::nil(),
            name: "x".to_string(),
            small_path: "".to_string(),
            medium_path: None,
            large_path: None,
        };
        assert!(row.try_into_photo().is_err());
    }

    #[test]
    fn try_into_photo_empty_optional_path_is_err() {
        let row = PhotoRow {
            id: Uuid::nil(),
            name: "x".to_string(),
            small_path: "photos/x_small.webp".to_string(),
            medium_path: Some("".to_string()),
            large_path: None,
        };
        assert!(row.try_into_photo().is_err());
    }

    #[test]
    fn from_photo_round_trip() {
        use rc_log_domain::asset::name::Name;
        use rc_log_domain::asset::path::Path;
        use rc_log_domain::asset::photo::{Photo, PhotoId};

        let photo = Photo::new(
            PhotoId::new(Uuid::nil()),
            Name::new("banner".to_string()).unwrap(),
            Path::new("photos/banner_small.webp".to_string()).unwrap(),
            Some(Path::new("photos/banner_medium.webp".to_string()).unwrap()),
            None,
        );
        let row = PhotoRow::from_photo(&photo);
        let roundtripped = row.try_into_photo().unwrap();
        assert_eq!(roundtripped.name.as_str(), photo.name.as_str());
        assert_eq!(roundtripped.small_path.as_str(), photo.small_path.as_str());
        assert_eq!(
            roundtripped.medium_path.as_ref().map(|p| p.as_str()),
            photo.medium_path.as_ref().map(|p| p.as_str()),
        );
        assert_eq!(
            roundtripped.large_path.as_ref().map(|p| p.as_str()),
            photo.large_path.as_ref().map(|p| p.as_str()),
        );
    }
}
