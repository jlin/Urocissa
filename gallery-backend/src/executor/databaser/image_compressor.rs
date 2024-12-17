use crate::{
    executor::databaser::{generate_dynamic_image::generate_dynamic_image, generate_exif::generate_exif},
    public::database_struct::database::definition::DataBase,
};

use image::{DynamicImage, ImageFormat};
use std::{error::Error, path::PathBuf};

use super::small_width_height;
pub fn image_compressor(
    database: &mut DataBase,
    dynamic_image_orientated: Option<DynamicImage>,
) -> Result<(), Box<dyn Error>> {
    let file_path = &database.imported_path();
    let (compressed_width, compressed_height) =
        small_width_height(database.width, database.height, 1280);
    if let Some(dynami_image) = dynamic_image_orientated {
        println!("case A");
        save_small_image(
            &dynami_image,
            compressed_width,
            compressed_height,
            database.compressed_path(),
        )?;
    } else {
        let exif_vec = generate_exif(&database);
        let dynamic_image = generate_dynamic_image(database)?;
        fix_orientation(database, &mut dynamic_image);
        println!("case B");
        save_small_image(
            &dynamic_image,
            compressed_width,
            compressed_height,
            database.compressed_path(),
        )?;
    }
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
