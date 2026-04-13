use std::path::PathBuf;

use tokio::fs;

// ─── Error ────────────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum AssetStorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

// ─── AssetStorage ─────────────────────────────────────────────────────────────

/// Filesystem-only storage for binary asset files.
///
/// All paths are relative to the configured `root` directory. The storage does
/// not interact with the database — it only reads and writes files.
#[derive(Clone)]
pub struct AssetStorage {
    root: PathBuf,
}

impl AssetStorage {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    /// Write `data` to `rel_path` (relative to the root), creating parent
    /// directories as needed.
    pub async fn save(
        &self,
        rel_path: &str,
        data: &[u8],
    ) -> Result<(), AssetStorageError> {
        let abs = self.root.join(rel_path);
        if let Some(parent) = abs.parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::write(abs, data).await?;
        Ok(())
    }

    /// Remove the file at `rel_path` (relative to the root).
    ///
    /// Returns `Ok(())` when the file does not exist — callers should treat a
    /// missing file as an already-completed delete.
    pub async fn delete(&self, rel_path: &str) -> Result<(), AssetStorageError> {
        let abs = self.root.join(rel_path);
        match fs::remove_file(&abs).await {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}
