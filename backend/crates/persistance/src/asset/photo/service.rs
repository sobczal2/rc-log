use rc_log_domain::asset::path::Path;
use rc_log_domain::asset::photo::{Photo, PhotoId};
use rc_log_domain::asset::photo_service::{PhotoService, PhotoServiceError};
use rc_log_domain::asset::photo::transaction::PhotoTransaction;
use rc_log_domain::shared::transaction::{Transaction, TransactionError};
use rc_log_domain::shared::unit_of_work::UnitOfWork;
use tracing::warn;
use super::super::asset_storage::{AssetStorage, AssetStorageError};
use super::super::processing::process_image;
use super::transaction::SqlxPhotoUnitOfWork;

#[derive(Clone)]
pub struct DiskDbPhotoService {
    photo_uow: SqlxPhotoUnitOfWork,
    asset_storage: AssetStorage,
}

impl DiskDbPhotoService {
    pub fn new(pool: sqlx::PgPool, asset_path: std::path::PathBuf) -> Self {
        Self {
            photo_uow: SqlxPhotoUnitOfWork::new(pool),
            asset_storage: AssetStorage::new(asset_path),
        }
    }

    fn rel_path(name: &str, suffix: &str) -> String {
        format!("photos/{}_{}.webp", name, suffix)
    }

    fn to_asset_path(rel: &str) -> Result<Path, PhotoServiceError> {
        Path::new(rel.to_string())
            .map_err(|e| PhotoServiceError::InvalidData(format!("Invalid asset path: {e}")))
    }

    fn map_tx_error(err: TransactionError) -> PhotoServiceError {
        match err {
            TransactionError::InvalidData(msg) => PhotoServiceError::InvalidData(msg),
            TransactionError::TransactionError(msg) => PhotoServiceError::DatabaseError(msg),
        }
    }

    fn map_asset_error(err: AssetStorageError) -> PhotoServiceError {
        match err {
            AssetStorageError::Io(e) => PhotoServiceError::IoError(e.to_string()),
        }
    }

    async fn cleanup_files_best_effort(&self, paths: &[String]) {
        for rel in paths {
            if let Err(e) = self.asset_storage.delete(rel).await {
                warn!(path = rel, error = %e, "Failed to cleanup photo file");
            }
        }
    }
}

impl PhotoService for DiskDbPhotoService {
    async fn save(&self, id: &PhotoId, data: &[u8]) -> Result<Photo, PhotoServiceError> {
        let data = data.to_vec();
        let id_str = id.as_uuid().to_string();

        let processed = tokio::task::spawn_blocking(move || process_image(&data))
            .await
            .map_err(|e| PhotoServiceError::IoError(format!("spawn_blocking error: {e}")))??;

        let mut tx_uow = self.photo_uow.clone();
        let mut tx = tx_uow.begin().await.map_err(Self::map_tx_error)?;

        let small_rel = Self::rel_path(&id_str, "small");
        self.asset_storage
            .save(&small_rel, &processed.small)
            .await
            .map_err(Self::map_asset_error)?;

        let medium_rel = match processed.medium {
            Some(ref bytes) => {
                let rel = Self::rel_path(&id_str, "medium");
                self.asset_storage.save(&rel, bytes).await.map_err(Self::map_asset_error)?;
                Some(rel)
            }
            None => None,
        };

        let large_rel = match processed.large {
            Some(ref bytes) => {
                let rel = Self::rel_path(&id_str, "large");
                self.asset_storage.save(&rel, bytes).await.map_err(Self::map_asset_error)?;
                Some(rel)
            }
            None => None,
        };

        let mut created_paths = vec![small_rel.clone()];
        if let Some(ref rel) = medium_rel {
            created_paths.push(rel.clone());
        }
        if let Some(ref rel) = large_rel {
            created_paths.push(rel.clone());
        }

        let photo = Photo::new(
            *id,
            Self::to_asset_path(&small_rel)?,
            medium_rel.as_deref().map(Self::to_asset_path).transpose()?,
            large_rel.as_deref().map(Self::to_asset_path).transpose()?,
        );

        if let Err(err) = tx.save(&photo).await {
            let _ = tx.rollback().await;
            return Err(Self::map_tx_error(err));
        }

        if let Err(err) = tx.commit().await {
            self.cleanup_files_best_effort(&created_paths).await;
            return Err(Self::map_tx_error(err));
        }

        Ok(photo)
    }

    async fn delete(&self, id: &PhotoId) -> Result<(), PhotoServiceError> {
        let mut tx_uow = self.photo_uow.clone();
        let mut tx = tx_uow.begin().await.map_err(Self::map_tx_error)?;

        let existing = tx.get_by_id(id).await.map_err(Self::map_tx_error)?;

        tx.delete_by_id(id).await.map_err(Self::map_tx_error)?;
        tx.commit().await.map_err(Self::map_tx_error)?;

        if let Some(photo) = existing {
            let mut paths = vec![photo.small_path.as_str().to_string()];
            if let Some(medium) = photo.medium_path {
                paths.push(medium.as_str().to_string());
            }
            if let Some(large) = photo.large_path {
                paths.push(large.as_str().to_string());
            }
            self.cleanup_files_best_effort(&paths).await;
        }

        Ok(())
    }
}
