use super::small_width_height;
use crate::structure::database_struct::database::definition::Database;
use anyhow::Context;
use anyhow::Result;
use image::{DynamicImage, ImageFormat};
use std::process::Command;
pub fn generate_thumbnail_for_image(
    database: &mut Database,
    dynamic_image: DynamicImage,
) -> Result<()> {
    let (compressed_width, compressed_height) =
        small_width_height(database.width, database.height, 1280);

    let thumbnail_image = dynamic_image
        .thumbnail_exact(compressed_width, compressed_height)
        .to_rgb8();

    let binding = database.compressed_path();
    let parent_path = binding.parent().ok_or_else(|| {
        anyhow::anyhow!(
            "image_compressor: failed to get parent directory for {:?}",
            database.compressed_path()
        )
    })?;

    std::fs::create_dir_all(parent_path)?;
    thumbnail_image.save_with_format(database.compressed_path(), ImageFormat::Jpeg)?;

    Ok(())
}

pub fn generate_thumbnail_for_video(database: &Database) -> Result<()> {
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
        let code = status.code().unwrap_or(-1); // If None, assign -1 or handle explicitly
        return Err(anyhow::anyhow!(
            "ffmpeg failed to generate thumbnail with exit code {}",
            code
        ));
    }
    Ok(())
}
