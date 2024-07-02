use anyhow::Result;
use camino::Utf8Path;
use image::ImageFormat;
use log::warn;
use oxipng::{optimize_from_memory, Options};
use tempfile::NamedTempFile;
use tracing::debug;

use crate::database::models::Formats;

/// Takes the path to an image file (`input`), applies some transformations and returns the path
/// to the transformed file (inside the provided `filesystem_local`).
///
/// Currently, it applies the following transformations:
///  * PNG files:
///     - Apply optimization using [`oxipng`] crate.
///
/// All the other formats are not transformed. The file is just copied to the output.
pub(crate) fn prepare_image_file(input: impl AsRef<Utf8Path>) -> Result<(NamedTempFile, Formats)> {
    let output = NamedTempFile::new()?;

    // Define fall-back behavior
    let fallback = || -> Result<Formats> {
        warn!("Cannot guess image format. Bypass any transformation and copy to the target destination");
        std::fs::copy(input.as_ref(), output.path())?;
        Ok(Formats::Unknown)
    };

    // Now run different operations based on format
    let image = image::io::Reader::open(input.as_ref())?.with_guessed_format()?;
    let format = match image.format() {
        Some(ImageFormat::Png) => {
            debug!("PNG: optimize using oxipng");
            let image = image.decode()?;
            match optimize_from_memory(image.as_bytes(), &Options::from_preset(2)) {
                Ok(vec) => {
                    let image = image::load_from_memory_with_format(&vec, ImageFormat::Png)?;
                    image.save_with_format(output.path(), ImageFormat::Png)?;
                    Formats::PNG
                }
                Err(e) => {
                    warn!("Error running oxipng optimizer: {}", e);
                    fallback()?
                }
            }
        }
        _ => fallback()?,
    };

    Ok((output, format))
}
