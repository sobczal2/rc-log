use std::io::Cursor;

use image::imageops::FilterType;
use image::{DynamicImage, ImageFormat};
use rc_log_domain::asset::photo_storage::PhotoStorageError;

// ─── Size constants ────────────────────────────────────────────────────────────

/// Longest-side pixel targets for each size tier.
pub(crate) const SMALL_PX: u32 = 400;
pub(crate) const MEDIUM_PX: u32 = 800;
pub(crate) const LARGE_PX: u32 = 1_600;

// ─── Pure image-processing layer (CPU-bound, no I/O) ──────────────────────────

/// WebP-encoded bytes for all size tiers produced from a single upload.
pub(crate) struct ProcessedPhoto {
    /// Always produced; longest side = [`SMALL_PX`].
    pub small: Vec<u8>,
    /// Present when source longest side ≥ [`SMALL_PX`]; longest side = [`MEDIUM_PX`].
    pub medium: Option<Vec<u8>>,
    /// Present when source longest side ≥ [`MEDIUM_PX`]; longest side = [`LARGE_PX`].
    pub large: Option<Vec<u8>>,
}

/// Decode `data`, produce up to three WebP size tiers, and return them.
///
/// |  Source longest side  | Tiers produced         | Each tier's longest side |
/// |-----------------------|------------------------|-------------------------|
/// | < 400 px              | small                  | 400 px (upscaled)        |
/// | 400 – 799 px          | small + medium         | 400 px / 800 px          |
/// | ≥ 800 px              | small + medium + large | 400 / 800 / 1600 px      |
///
/// Every tier is always resized to exactly its target size (upscaling included).
pub(crate) fn process_image(data: &[u8]) -> Result<ProcessedPhoto, PhotoStorageError> {
    let img = image::load_from_memory(data)
        .map_err(|e| PhotoStorageError::InvalidData(format!("Decode error: {e}")))?;

    let longest = img.width().max(img.height());

    let small = encode_webp(&resize_to(&img, SMALL_PX))?;

    let medium =
        (longest >= SMALL_PX).then(|| encode_webp(&resize_to(&img, MEDIUM_PX))).transpose()?;

    let large =
        (longest >= MEDIUM_PX).then(|| encode_webp(&resize_to(&img, LARGE_PX))).transpose()?;

    Ok(ProcessedPhoto { small, medium, large })
}

/// Resize `img` so its longest side equals `target_px`, preserving aspect ratio.
/// Both downscaling and upscaling are performed as needed.
fn resize_to(img: &DynamicImage, target_px: u32) -> DynamicImage {
    let longest = img.width().max(img.height());
    let scale = target_px as f64 / longest as f64;
    let new_w = (img.width() as f64 * scale).round() as u32;
    let new_h = (img.height() as f64 * scale).round() as u32;
    img.resize_exact(new_w, new_h, FilterType::Lanczos3)
}

/// Encode `img` to lossy WebP bytes.
fn encode_webp(img: &DynamicImage) -> Result<Vec<u8>, PhotoStorageError> {
    let mut buf = Cursor::new(Vec::new());
    img.write_to(&mut buf, ImageFormat::WebP)
        .map_err(|e| PhotoStorageError::InvalidData(format!("WebP encode error: {e}")))?;
    Ok(buf.into_inner())
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
    fn small_only_when_image_below_400() {
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
    fn all_sizes_when_800_or_larger() {
        let result = process_image(&make_png(1200, 900)).unwrap();
        assert!(result.medium.is_some());
        assert!(result.large.is_some());
    }

    #[test]
    fn exactly_400px_produces_small_and_medium() {
        let result = process_image(&make_png(400, 300)).unwrap();
        assert!(result.medium.is_some());
        assert!(result.large.is_none());
    }

    #[test]
    fn exactly_800px_produces_all_three() {
        let result = process_image(&make_png(800, 600)).unwrap();
        assert!(result.medium.is_some());
        assert!(result.large.is_some());
    }

    // ── Output dimensions ─────────────────────────────────────────────────────

    #[test]
    fn small_longest_side_is_400_when_downscaling() {
        let result = process_image(&make_png(800, 600)).unwrap();
        let (w, h) = webp_dims(&result.small);
        assert_eq!(w.max(h), SMALL_PX);
    }

    #[test]
    fn small_longest_side_is_400_when_upscaling() {
        let result = process_image(&make_png(200, 150)).unwrap();
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
