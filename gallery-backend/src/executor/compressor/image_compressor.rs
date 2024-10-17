use crate::public::database_struct::database::definition::DataBase;

use super::{image_thumbhash::generate_thumbhash, utils::small_width_height};

use image::{DynamicImage, ImageFormat};
use std::{error::Error, path::PathBuf};
pub fn image_compressor(database: &mut DataBase) -> Result<(), Box<dyn Error>> {
    let file_path = &database.imported_path();
    let (width, height, dynamic_image) = generate_thumbhash(database, file_path)?;
    let (compressed_width, compressed_height) = small_width_height(width, height, 1280);
    save_small_image(
        &dynamic_image,
        compressed_width,
        compressed_height,
        database.compressed_path(),
    )?;
    Ok(())
}

pub fn save_small_image(
    dynamic_image: &DynamicImage,
    nwidth: u32,
    nheight: u32,
    save_path: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let preview_image = dynamic_image.thumbnail_exact(nwidth, nheight).to_rgb8();
    let parent_path = &save_path.parent().ok_or_else(|| {
        format!(
            "video_compressor: failed to get parent directory for {:?}",
            &save_path
        )
    })?;

    std::fs::create_dir_all(parent_path)?;
    let result = preview_image.save_with_format(&save_path, ImageFormat::Jpeg);
    match result {
        Ok(()) => Ok(()),
        Err(err) => Err(Box::new(err)),
    }
}
