//! The Workshop shows previews as 512x512 squares and refuses files of
//! 1 MB or more, so still images are center-cropped, resized and saved
//! as JPEG. GIFs pass untouched: re-encoding would drop the animation.

mod info;

use std::path::{Path, PathBuf};

pub use info::{describe, PreviewInfo};

use image::{imageops::FilterType, DynamicImage, ImageReader, Limits};

use crate::WorkshopError;

pub const PREVIEW_SIDE: u32 = 512;
/// Steam's limit (exclusive).
pub const PREVIEW_MAX: u64 = 1024 * 1024;
/// Larger inputs are almost surely a mistake and would cost a lot to decode.
pub const SOURCE_MAX: u64 = 32 * 1024 * 1024;

/// The path to upload: `path` itself when it already fits, else a
/// prepared copy written into `work`.
pub fn prepare_preview(path: &Path, work: &Path) -> Result<PathBuf, WorkshopError> {
    let size = std::fs::metadata(path)
        .map_err(|e| WorkshopError::io(path, e))?
        .len();
    if path
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("gif"))
    {
        return Ok(path.to_path_buf());
    }
    let img = decode(path)?;
    let fits = img.width() == PREVIEW_SIDE && img.height() == PREVIEW_SIDE;
    if fits && size < PREVIEW_MAX {
        return Ok(path.to_path_buf());
    }
    let out = work.join("preview.jpg");
    square(&img)
        .to_rgb8()
        .save_with_format(&out, image::ImageFormat::Jpeg)
        .map_err(|e| WorkshopError::invalid("preview", e.to_string()))?;
    Ok(out)
}

/// Bounded decode: a tiny file can claim a huge canvas.
fn decode(path: &Path) -> Result<DynamicImage, WorkshopError> {
    let bad = |e: String| WorkshopError::invalid("preview", e);
    let mut reader = ImageReader::open(path)
        .map_err(|e| WorkshopError::io(path, e))?
        .with_guessed_format()
        .map_err(|e| WorkshopError::io(path, e))?;
    let mut limits = Limits::default();
    limits.max_image_width = Some(8192);
    limits.max_image_height = Some(8192);
    limits.max_alloc = Some(512 * 1024 * 1024);
    reader.limits(limits);
    reader.decode().map_err(|e| bad(e.to_string()))
}

/// Center crop to a square, then scale to the Workshop's side.
fn square(img: &DynamicImage) -> DynamicImage {
    let side = img.width().min(img.height()).max(1);
    let x = (img.width() - side) / 2;
    let y = (img.height() - side) / 2;
    img.crop_imm(x, y, side, side)
        .resize_exact(PREVIEW_SIDE, PREVIEW_SIDE, FilterType::CatmullRom)
}
