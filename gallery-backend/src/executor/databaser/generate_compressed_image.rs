use crate::{
    executor::databaser::{
        generate_dynamic_image::generate_dynamic_image, generate_exif::generate_image_exif,
    },
    public::database_struct::database::definition::DataBase,
};
use image::{DynamicImage, ImageFormat};
use std::error::Error;

use super::{fix_orientation::fix_image_orientation, small_width_height};

pub fn generate_compressed_image(
    database: &mut DataBase,
    dynamic_image_orientated: Option<DynamicImage>,
) -> Result<(), Box<dyn Error>> {
    let (compressed_width, compressed_height) =
        small_width_height(database.width, database.height, 1280);
    let dynamic_image = if let Some(dyn_image) = dynamic_image_orientated {
        dyn_image
    } else {
        generate_image_exif(&database);
        let mut dyn_img = generate_dynamic_image(database)?;
        fix_image_orientation(database, &mut dyn_img);
        dyn_img
    };

    let preview_image = dynamic_image
        .thumbnail_exact(compressed_width, compressed_height)
        .to_rgb8();

    let binding = database.compressed_path();
    let parent_path = binding.parent().ok_or_else(|| {
        format!(
            "image_compressor: failed to get parent directory for {:?}",
            database.compressed_path()
        )
    })?;

    std::fs::create_dir_all(parent_path)?;
    preview_image.save_with_format(database.compressed_path(), ImageFormat::Jpeg)?;

    Ok(())
}
