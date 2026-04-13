use std::path::PathBuf;

use rc_log_domain::asset::name::AssetName;
use rc_log_domain::asset::path::AssetPath;
use rc_log_domain::asset::photo::{Photo, PhotoId};
use rc_log_domain::asset::photo_storage::{PhotoStorage, PhotoStorageError};
use sqlx::PgPool;
use tokio::fs;
use uuid::Uuid;

use super::processing::process_image;

// ─── Disk + DB storage ────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct DiskDbPhotoStorage {
    pool: PgPool,
    asset_path: PathBuf,
}

impl DiskDbPhotoStorage {
    pub fn new(pool: PgPool, asset_path: PathBuf) -> Self {
        Self { pool, asset_path }
    }

    fn rel_path(name: &str, suffix: &str) -> String {
        format!("photos/{}_{}.webp", name, suffix)
    }

    fn to_asset_path(rel: &str) -> Result<AssetPath, PhotoStorageError> {
        AssetPath::new(rel.to_string())
            .map_err(|e| PhotoStorageError::InvalidData(format!("Invalid asset path: {e}")))
    }
}

impl PhotoStorage for DiskDbPhotoStorage {
    async fn store(&self, name: &AssetName, data: &[u8]) -> Result<Photo, PhotoStorageError> {
        let data = data.to_vec();
        let name_str = name.as_str().to_string();

        // All CPU-bound work (decode, resize, encode) runs off the async executor.
        let processed = tokio::task::spawn_blocking(move || process_image(&data))
            .await
            .map_err(|e| PhotoStorageError::IoError(format!("spawn_blocking error: {e}")))??;

        let photos_dir = self.asset_path.join("photos");
        fs::create_dir_all(&photos_dir)
            .await
            .map_err(|e| PhotoStorageError::IoError(format!("create_dir_all: {e}")))?;

        let small_rel = Self::rel_path(&name_str, "small");
        fs::write(self.asset_path.join(&small_rel), &processed.small)
            .await
            .map_err(|e| PhotoStorageError::IoError(format!("write small: {e}")))?;

        let medium_rel = match processed.medium {
            Some(ref bytes) => {
                let rel = Self::rel_path(&name_str, "medium");
                fs::write(self.asset_path.join(&rel), bytes)
                    .await
                    .map_err(|e| PhotoStorageError::IoError(format!("write medium: {e}")))?;
                Some(rel)
            }
            None => None,
        };

        let large_rel = match processed.large {
            Some(ref bytes) => {
                let rel = Self::rel_path(&name_str, "large");
                fs::write(self.asset_path.join(&rel), bytes)
                    .await
                    .map_err(|e| PhotoStorageError::IoError(format!("write large: {e}")))?;
                Some(rel)
            }
            None => None,
        };

        let photo_id = PhotoId::new(Uuid::new_v4());
        let small_path = Self::to_asset_path(&small_rel)?;
        let medium_path = medium_rel.as_deref().map(Self::to_asset_path).transpose()?;
        let large_path = large_rel.as_deref().map(Self::to_asset_path).transpose()?;

        sqlx::query(
            r#"
            INSERT INTO asset.photo (id, name, small_path, medium_path, large_path)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (name) DO UPDATE
                SET small_path  = EXCLUDED.small_path,
                    medium_path = EXCLUDED.medium_path,
                    large_path  = EXCLUDED.large_path
            "#,
        )
        .bind(Uuid::from(photo_id))
        .bind(name.as_str())
        .bind(small_path.as_str())
        .bind(medium_path.as_ref().map(|p| p.as_str()))
        .bind(large_path.as_ref().map(|p| p.as_str()))
        .execute(&self.pool)
        .await
        .map_err(|e| PhotoStorageError::DatabaseError(format!("upsert photo: {e}")))?;

        Ok(Photo::new(photo_id, name.clone(), small_path, medium_path, large_path))
    }

    async fn delete(&self, name: &AssetName) -> Result<(), PhotoStorageError> {
        #[derive(sqlx::FromRow)]
        struct PathsRow {
            small_path: String,
            medium_path: Option<String>,
            large_path: Option<String>,
        }

        let row: Option<PathsRow> = sqlx::query_as(
            r#"SELECT small_path, medium_path, large_path FROM asset.photo WHERE name = $1"#,
        )
        .bind(name.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| PhotoStorageError::DatabaseError(format!("fetch paths: {e}")))?;

        sqlx::query(r#"DELETE FROM asset.photo WHERE name = $1"#)
            .bind(name.as_str())
            .execute(&self.pool)
            .await
            .map_err(|e| PhotoStorageError::DatabaseError(format!("delete photo: {e}")))?;

        if let Some(row) = row {
            for rel_path in
                [Some(row.small_path), row.medium_path, row.large_path].into_iter().flatten()
            {
                let abs = self.asset_path.join(&rel_path);
                if let Err(e) = fs::remove_file(&abs).await {
                    if e.kind() != std::io::ErrorKind::NotFound {
                        tracing::warn!(
                            path = %abs.display(),
                            error = %e,
                            "Failed to delete photo file"
                        );
                    }
                }
            }
        }

        Ok(())
    }
}


