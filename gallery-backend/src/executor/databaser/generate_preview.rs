use crate::public::database_struct::database::definition::DataBase;
use anyhow::Context;
use std::{error::Error, process::Command};

use super::small_width_height;

pub fn generate_thumbnail(database: &DataBase) -> Result<(), Box<dyn Error>> {
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
