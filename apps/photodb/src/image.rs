use anyhow::anyhow;
use camino::Utf8Path;
use oxipng::{optimize, Options};
use tracing::debug;

use filesystem::impls::FilesystemLocalTemp;
use filesystem::FilePathBuf;

/// Takes the path to an image file (`input`), applies some transformations and returns the path
/// to the transformed file (inside the provided `filesystem_local`).
///
/// Currently, the transformation it applies are:
///  * Convert to PNG using the [`image`] crate.
///  * Apply optimization using [`oxipng`] crate.
pub(crate) fn prepare_image_file(
    input: impl AsRef<Utf8Path>,
    filesystem_local: &FilesystemLocalTemp,
) -> anyhow::Result<FilePathBuf> {
    debug!("Convert to PNG format");
    let input = {
        let image = image::io::Reader::open(input.as_ref())?.decode()?;
        let input_filename = tempfile::NamedTempFile::new()?.into_temp_path();
        image.save_with_format(&input_filename, image::ImageFormat::Png)?;
        input_filename
    };

    debug!("Apply oxipng optimizer");
    let output_filename = {
        let input_file = oxipng::InFile::Path(input.to_path_buf());

        let tmp_filename = filesystem_local.temp_filename(None, None);
        let fullpath_tmp_filename = filesystem_local.resolve_filepath(&tmp_filename);
        let output_file = oxipng::OutFile::from_path(fullpath_tmp_filename.into_std_path_buf());

        optimize(&input_file, &output_file, &Options::from_preset(2))
            .map_err(|e| anyhow!("Error converting image: {e}"))?;
        debug!("Input photo optimized and saved");
        tmp_filename
    };

    Ok(output_filename)
}
