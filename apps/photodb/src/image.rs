use anyhow::Result;
use camino::Utf8Path;
use camino_tempfile::NamedUtf8TempFile;
use image::ImageFormat;
use log::warn;
use oxipng::{optimize_from_memory, Options};
use tracing::debug;

use crate::database::models::Formats;

/// Takes the path to an image file (`input`), applies some transformations and returns the path
/// to the transformed file.
///
/// Currently, it applies the following transformations:
///  * PNG files:
///     - Apply optimization using [`oxipng`] crate.
///
/// All the other formats are not transformed. The file is just copied to the output.
pub(crate) fn prepare_image_file(input: impl AsRef<Utf8Path>) -> Result<(NamedUtf8TempFile, Formats)> {
    let output = NamedUtf8TempFile::new()?;

    // Define fall-back behavior: do nothing and copy to a temporary file
    let fallback = |format: Formats| -> Result<Formats> {
        std::fs::copy(input.as_ref(), output.path())?;
        Ok(format)
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
                    Formats::Png
                }
                Err(e) => {
                    warn!("Error running oxipng optimizer: {}", e);
                    fallback(Formats::Png)? // FIXME: If optimizer fails, is this a PNG?
                }
            }
        }
        Some(f) => {
            debug!("Nothing to do for format {:?}", f);
            fallback(f.into())?
        }
        _ => {
            warn!("Cannot guess file format. Bypass transformation and copy to the target destination");
            fallback(Formats::Unknown)?
        }
    };

    Ok((output, format))
}
