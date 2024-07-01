use camino::Utf8Path;
use image::ImageFormat;
use log::warn;
use oxipng::{optimize_from_memory, Options};
use tracing::debug;

use filesystem::impls::FilesystemLocalTemp;
use filesystem::FilePathBuf;

/// Takes the path to an image file (`input`), applies some transformations and returns the path
/// to the transformed file (inside the provided `filesystem_local`).
///
/// Currently, it applies the following transformations:
///  * Convert to PNG using the [`image`] crate.
///  * Apply optimization using [`oxipng`] crate.
pub(crate) fn prepare_image_file(
    input: impl AsRef<Utf8Path>,
    filesystem_local: &FilesystemLocalTemp,
) -> anyhow::Result<FilePathBuf> {
    debug!("Collect EXIF data");
    let file = std::fs::File::open(input.as_ref())?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader)?;
    for f in exif.fields() {
        println!("{} -{}- {}", f.tag, f.ifd_num, f.display_value().with_unit(&exif));
    }

    let tmp_filename = filesystem_local.temp_filename(None, None);
    let fullpath_tmp_filename = filesystem_local.resolve_filepath(&tmp_filename);

    // Now run different operations based on format
    let image = image::io::Reader::open(input.as_ref())?.decode()?;
    match image::guess_format(image.as_bytes()) {
        Ok(format) => {
            debug!("Format guessed as {:?} format", format);
            match format {
                ImageFormat::Png => {
                    debug!("PNG: optimize using oxipng");
                    let vec = optimize_from_memory(image.as_bytes(), &Options::from_preset(2))?;
                    let image = image::load_from_memory_with_format(&vec, format)?;
                    image.save_with_format(fullpath_tmp_filename, format)?;
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
                    std::fs::copy(input.as_ref(), fullpath_tmp_filename)?;
                }
            }
        }
        Err(e) => {
            warn!("Cannot guess image format. Bypass any transformation and copy to the target destination: {e}");
            std::fs::copy(input.as_ref(), fullpath_tmp_filename)?;
        }
    };

    Ok(tmp_filename)
}
