use rc_log_domain::asset::path::Path;
use rc_log_domain::asset::video::transaction::VideoTransaction;
use rc_log_domain::asset::video::{Video, VideoId};
use rc_log_domain::shared::transaction::{Transaction, TransactionError};
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use sqlx::{PgPool, Postgres, Transaction as SqlxTransaction};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
struct VideoRow {
    id: Uuid,
    small_path: String,
    medium_path: Option<String>,
    large_path: Option<String>,
}

impl VideoRow {
    fn try_into_video(self) -> Result<Video, TransactionError> {
        let small_path =
            Path::new(self.small_path).map_err(|e| TransactionError::InvalidData(e.to_string()))?;
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

        Ok(Video::new(VideoId::new(self.id), small_path, medium_path, large_path))
    }

    fn from_video(video: &Video) -> Self {
        Self {
            id: Uuid::from(video.id),
            small_path: video.small_path.as_str().to_string(),
            medium_path: video.medium_path.as_ref().map(|p| p.as_str().to_string()),
            large_path: video.large_path.as_ref().map(|p| p.as_str().to_string()),
        }
    }
}

pub struct SqlxVideoTransaction {
    tx: SqlxTransaction<'static, Postgres>,
}

impl Transaction<Video> for SqlxVideoTransaction {
    async fn save(&mut self, video: &Video) -> Result<(), TransactionError> {
        let row = VideoRow::from_video(video);

        sqlx::query(
            r#"
            INSERT INTO asset.video (id, small_path, medium_path, large_path)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id) DO UPDATE SET
                small_path  = EXCLUDED.small_path,
                medium_path = EXCLUDED.medium_path,
                large_path  = EXCLUDED.large_path
            "#,
        )
        .bind(row.id)
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

impl VideoTransaction for SqlxVideoTransaction {
    async fn get_by_id(&mut self, id: &VideoId) -> Result<Option<Video>, TransactionError> {
        let row: Option<VideoRow> = sqlx::query_as(
            r#"
            SELECT id, small_path, medium_path, large_path
            FROM asset.video
            WHERE id = $1
            "#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        row.map(VideoRow::try_into_video).transpose()
    }

    async fn delete_by_id(&mut self, id: &VideoId) -> Result<(), TransactionError> {
        sqlx::query(
            r#"
            DELETE FROM asset.video
            WHERE id = $1
            "#,
        )
        .bind(id.as_uuid())
        .execute(&mut *self.tx)
        .await
        .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        Ok(())
    }
}

#[derive(Clone)]
pub struct SqlxVideoUnitOfWork {
    pool: PgPool,
}

impl SqlxVideoUnitOfWork {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UnitOfWork<Video> for SqlxVideoUnitOfWork {
    type Transaction = SqlxVideoTransaction;

    async fn begin(&mut self) -> Result<Self::Transaction, TransactionError> {
        let tx = self
            .pool
            .begin()
            .await
            .map_err(|e| TransactionError::TransactionError(e.to_string()))?;

        Ok(SqlxVideoTransaction { tx })
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
            small_path: "videos/hero_small.mp4".to_string(),
            medium_path: Some("videos/hero_medium.mp4".to_string()),
            large_path: Some("videos/hero_large.mp4".to_string()),
        };
        let video = row.try_into_video().unwrap();
        assert!(video.medium_path.is_some());
        assert!(video.large_path.is_some());
    }

    #[test]
    fn try_into_video_valid_small_only() {
        let row = VideoRow {
            id: Uuid::nil(),
            small_path: "videos/thumb_small.mp4".to_string(),
            medium_path: None,
            large_path: None,
        };
        let video = row.try_into_video().unwrap();
        assert!(video.medium_path.is_none());
        assert!(video.large_path.is_none());
    }

    #[test]
    fn try_into_video_empty_small_path_is_err() {
        let row = VideoRow {
            id: Uuid::nil(),
            small_path: "".to_string(),
            medium_path: None,
            large_path: None,
        };
        assert!(row.try_into_video().is_err());
    }

    #[test]
    fn try_into_video_empty_optional_path_is_err() {
        let row = VideoRow {
            id: Uuid::nil(),
            small_path: "videos/x_small.mp4".to_string(),
            medium_path: Some("".to_string()),
            large_path: None,
        };
        assert!(row.try_into_video().is_err());
    }

    #[test]
    fn from_video_round_trip() {
        use rc_log_domain::asset::path::Path;
        use rc_log_domain::asset::video::{Video, VideoId};

        let video = Video::new(
            VideoId::new(Uuid::nil()),
            Path::new("videos/hero_small.mp4".to_string()).unwrap(),
            Some(Path::new("videos/hero_medium.mp4".to_string()).unwrap()),
            None,
        );

        let row = VideoRow::from_video(&video);
        let roundtripped = row.try_into_video().unwrap();
        assert_eq!(roundtripped.small_path.as_str(), video.small_path.as_str());
        assert_eq!(
            roundtripped.medium_path.as_ref().map(|p| p.as_str()),
            video.medium_path.as_ref().map(|p| p.as_str()),
        );
        assert_eq!(
            roundtripped.large_path.as_ref().map(|p| p.as_str()),
            video.large_path.as_ref().map(|p| p.as_str()),
        );
    }
}
