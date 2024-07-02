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

    // Now run different operations based on format
    let image = image::io::Reader::open(input.as_ref())?.decode()?;
    let format = match image::guess_format(image.as_bytes()) {
        Ok(format) => {
            debug!("Format guessed as {:?} format", format);
            match format {
                ImageFormat::Png => {
                    debug!("PNG: optimize using oxipng");
                    let vec = optimize_from_memory(image.as_bytes(), &Options::from_preset(2))?;
                    let image = image::load_from_memory_with_format(&vec, format)?;
                    image.save_with_format(output.path(), format)?;
                    Formats::PNG
                }
                // ImageFormat::Jpeg => {}
                // ImageFormat::Gif => {}
                // ImageFormat::WebP => {}
                // ImageFormat::Pnm => {}
                // ImageFormat::Tiff => {}
                // ImageFormat::Tga => {}
                // ImageFormat::Dds => {}
                // ImageFormat::Bmp => {}
                // ImageFormat::Ico => {}
                // ImageFormat::Hdr => {}
                // ImageFormat::OpenExr => {}
                // ImageFormat::Farbfeld => {}
                // ImageFormat::Avif => {}
                // ImageFormat::Qoi => {}
                _ => {
                    warn!("Logic for this format is not implemented. Bypass any transformation");
                    std::fs::copy(input.as_ref(), output.path())?;
                    Formats::Unknown
                }
            }
        }
        Err(e) => {
            warn!("Cannot guess image format. Bypass any transformation and copy to the target destination: {e}");
            std::fs::copy(input.as_ref(), output.path())?;
            Formats::Unknown
        }
    };

    Ok((output, format))
}
