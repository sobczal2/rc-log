use std::io::Cursor;
use std::path::PathBuf;

use image::imageops::FilterType;
use image::{DynamicImage, ImageFormat};
use rc_log_domain::asset::name::AssetName;
use rc_log_domain::asset::path::AssetPath;
use rc_log_domain::asset::photo::{Photo, PhotoId};
use rc_log_domain::asset::photo_storage::{PhotoStorage, PhotoStorageError};
use sqlx::PgPool;
use tokio::fs;
use uuid::Uuid;

// ─── Size constants ────────────────────────────────────────────────────────────

/// Longest-side pixel targets for each size tier.
const SMALL_PX: u32 = 400;
const MEDIUM_PX: u32 = 800;
const LARGE_PX: u32 = 1_600;

// ─── Pure image-processing layer (CPU-bound, no I/O) ──────────────────────────

/// WebP-encoded bytes for all size tiers produced from a single upload.
pub(crate) struct ProcessedPhoto {
    /// Always produced; longest side ≤ [`SMALL_PX`].
    pub small: Vec<u8>,
    /// Present when source longest side > [`SMALL_PX`]; longest side ≤ [`MEDIUM_PX`].
    pub medium: Option<Vec<u8>>,
    /// Present when source longest side > [`MEDIUM_PX`]; longest side ≤ [`LARGE_PX`].
    pub large: Option<Vec<u8>>,
}

/// Decode `data`, produce up to three WebP size tiers, and return them.
///
/// |  Source longest side  | Tiers produced         |
/// |-----------------------|------------------------|
/// | ≤ 400 px              | small                  |
/// | 401 – 800 px          | small + medium         |
/// | > 800 px              | small + medium + large |
///
/// Images are never upscaled — if the source is already smaller than a tier's
/// target, that tier receives the source at its natural dimensions.
pub(crate) fn process_image(data: &[u8]) -> Result<ProcessedPhoto, PhotoStorageError> {
    let img = image::load_from_memory(data)
        .map_err(|e| PhotoStorageError::InvalidData(format!("Decode error: {e}")))?;

    let longest = img.width().max(img.height());

    let small = encode_webp(&resize_to(&img, SMALL_PX))?;

    let medium =
        (longest > SMALL_PX).then(|| encode_webp(&resize_to(&img, MEDIUM_PX))).transpose()?;

    let large =
        (longest > MEDIUM_PX).then(|| encode_webp(&resize_to(&img, LARGE_PX))).transpose()?;

    Ok(ProcessedPhoto { small, medium, large })
}

/// Resize `img` so its longest side is at most `max_px`, preserving aspect ratio.
/// Returns a clone unchanged when the image already fits (no upscaling).
fn resize_to(img: &DynamicImage, max_px: u32) -> DynamicImage {
    let longest = img.width().max(img.height());
    if longest <= max_px {
        return img.clone();
    }
    let scale = max_px as f64 / longest as f64;
    let new_w = (img.width() as f64 * scale).round() as u32;
    let new_h = (img.height() as f64 * scale).round() as u32;
    img.resize(new_w, new_h, FilterType::Lanczos3)
}

/// Encode `img` to lossy WebP bytes.
fn encode_webp(img: &DynamicImage) -> Result<Vec<u8>, PhotoStorageError> {
    let mut buf = Cursor::new(Vec::new());
    img.write_to(&mut buf, ImageFormat::WebP)
        .map_err(|e| PhotoStorageError::InvalidData(format!("WebP encode error: {e}")))?;
    Ok(buf.into_inner())
}

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

// ─── Unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// Encode a blank RGBA image of the given dimensions to PNG bytes.
    fn make_png(width: u32, height: u32) -> Vec<u8> {
        let img = DynamicImage::ImageRgba8(image::RgbaImage::new(width, height));
        let mut buf = Cursor::new(Vec::new());
        img.write_to(&mut buf, ImageFormat::Png).unwrap();
        buf.into_inner()
    }

    /// Decode WebP bytes and return `(width, height)`.
    fn webp_dims(bytes: &[u8]) -> (u32, u32) {
        let img = image::load_from_memory(bytes).expect("valid WebP output");
        (img.width(), img.height())
    }

    // ── Tier selection ────────────────────────────────────────────────────────

    #[test]
    fn small_only_when_image_fits_in_400() {
        let result = process_image(&make_png(300, 200)).unwrap();
        assert!(result.medium.is_none());
        assert!(result.large.is_none());
    }

    #[test]
    fn small_and_medium_when_between_400_and_800() {
        let result = process_image(&make_png(600, 400)).unwrap();
        assert!(result.medium.is_some());
        assert!(result.large.is_none());
    }

    #[test]
    fn all_sizes_when_larger_than_800() {
        let result = process_image(&make_png(1200, 900)).unwrap();
        assert!(result.medium.is_some());
        assert!(result.large.is_some());
    }

    // ── Output dimensions ─────────────────────────────────────────────────────

    #[test]
    fn small_longest_side_is_400() {
        let result = process_image(&make_png(800, 600)).unwrap();
        let (w, h) = webp_dims(&result.small);
        assert_eq!(w.max(h), SMALL_PX);
    }

    #[test]
    fn medium_longest_side_is_800() {
        let result = process_image(&make_png(1600, 1000)).unwrap();
        let (w, h) = webp_dims(result.medium.as_ref().unwrap());
        assert_eq!(w.max(h), MEDIUM_PX);
    }

    #[test]
    fn large_longest_side_is_1600() {
        let result = process_image(&make_png(3200, 2400)).unwrap();
        let (w, h) = webp_dims(result.large.as_ref().unwrap());
        assert_eq!(w.max(h), LARGE_PX);
    }

    #[test]
    fn small_image_is_not_upscaled() {
        let result = process_image(&make_png(200, 150)).unwrap();
        let (w, h) = webp_dims(&result.small);
        // Source is below SMALL_PX — must not be upscaled.
        assert_eq!((w, h), (200, 150));
    }

    // ── Aspect ratio ──────────────────────────────────────────────────────────

    #[test]
    fn aspect_ratio_preserved_in_small() {
        // 2:1 landscape — after resize longest side = 400, shorter side ≈ 200.
        let result = process_image(&make_png(800, 400)).unwrap();
        let (w, h) = webp_dims(&result.small);
        assert_eq!(w.max(h), SMALL_PX);
        let ratio = w as f64 / h as f64;
        assert!((ratio - 2.0).abs() < 0.1, "expected ~2:1, got {w}×{h}");
    }

    // ── Error handling ────────────────────────────────────────────────────────

    #[test]
    fn invalid_bytes_return_error() {
        let result = process_image(b"not an image");
        assert!(matches!(result, Err(PhotoStorageError::InvalidData(_))));
    }
}
