use rc_log_domain::video::path::Path;
use rc_log_domain::video::transaction::VideoTransaction;
use rc_log_domain::video::{Video, VideoId};
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
