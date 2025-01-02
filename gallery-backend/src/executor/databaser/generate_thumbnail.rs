use crate::public::database_struct::database::definition::DataBase;
use anyhow::Context;
use std::{error::Error, process::Command};

use super::{generate_exif::generate_exif_for_image, small_width_height};

use crate::executor::databaser::generate_dynamic_image::generate_dynamic_image;
use image::{DynamicImage, ImageFormat};

use super::fix_orientation::fix_image_orientation;

pub fn generate_thumbnail_for_image(
    database: &mut DataBase,
    dynamic_image: DynamicImage,
) -> Result<(), Box<dyn Error>> {
    let (compressed_width, compressed_height) =
        small_width_height(database.width, database.height, 1280);

    let thumbnail_image = dynamic_image
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
    thumbnail_image.save_with_format(database.compressed_path(), ImageFormat::Jpeg)?;

    Ok(())
}

pub fn generate_thumbnail_for_video(database: &DataBase) -> Result<(), Box<dyn Error>> {
    let width = database.width;
    let height = database.height;

    let (thumbnail_width, thumbnail_height) = small_width_height(width, height, 1280);

    let thumbnail_scale_args = format!("scale={}:{}", thumbnail_width, thumbnail_height);

    let thumbnail_file_path_string = &database.thumbnail_path();
    std::fs::create_dir_all(database.compressed_path_parent()).with_context(|| {
        format!(
            "generate_thumbnail: failed to create directory for {:?}",
            database.imported_path_string()
        )
    })?;
    let status = Command::new("ffmpeg")
        .args(&[
            "-y",
            "-i",
            &database.imported_path_string(),
            "-ss",
            "0",
            "-frames:v",
            "1", // Generate only one image
            "-vf",
            &thumbnail_scale_args,
            thumbnail_file_path_string,
        ])
        .status()
        .with_context(|| {
            format!(
                "generate_thumbnail: failed to spawn new command for ffmpeg: {:?}",
                thumbnail_file_path_string
            )
        })?;

    if !status.success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "ffmpeg failed to generate thumbnail",
        )));
    }
    Ok(())
}

pub fn regenerate_compressed_image(database: &mut DataBase) -> Result<(), Box<dyn Error>> {
    let (compressed_width, compressed_height) =
        small_width_height(database.width, database.height, 1280);
    let dynamic_image = {
        // To ensure that the exif_vec is accurate, regenerate a new one from imported path
        database.exif_vec = generate_exif_for_image(&database);
        let mut dyn_img = generate_dynamic_image(database)?;
        fix_image_orientation(database, &mut dyn_img);
        dyn_img
    };

    let thumbnail_image = dynamic_image
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
    thumbnail_image.save_with_format(database.compressed_path(), ImageFormat::Jpeg)?;

    Ok(())
}
