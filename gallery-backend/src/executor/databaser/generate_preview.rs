use crate::public::database_struct::database::definition::DataBase;
use anyhow::Context;
use std::{error::Error, process::Command};

use super::small_width_height;

pub fn generate_preview(database: &DataBase) -> Result<(), Box<dyn Error>> {
    let width = database.width;
    let height = database.height;

    let (preview_width, preview_height) = small_width_height(width, height, 1280);

    let preview_scale_args = format!("scale={}:{}", preview_width, preview_height);

    let preview_file_path_string = &database.preview_path();
    std::fs::create_dir_all(database.compressed_path_parent()).with_context(|| {
        format!(
            "generate_preview: failed to create directory for {:?}",
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
            &preview_scale_args,
            preview_file_path_string,
        ])
        .status()
        .with_context(|| {
            format!(
                "generate_preview: failed to spawn new command for ffmpeg: {:?}",
                preview_file_path_string
            )
        })?;

    if !status.success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "ffmpeg failed to generate preview",
        )));
    }
    Ok(())
}

pub fn generate_preview_by_current_frame(
    database: &DataBase,
    current_frame_second: f64,
) -> Result<(), Box<dyn Error>> {
    let width = database.width;
    let height = database.height;

    let (preview_width, preview_height) = small_width_height(width, height, 1280);

    let preview_scale_args = format!("scale={}:{}", preview_width, preview_height);

    let preview_file_path_string = database.preview_path();
    std::fs::create_dir_all(database.compressed_path_parent()).with_context(|| {
        format!(
            "generate_preview_by_current_frame: failed to create directory for {:?}",
            database.imported_path_string()
        )
    })?;
    let status = Command::new("ffmpeg")
        .args(&[
            "-y",
            "-i",
            &database.imported_path_string(),
            "-ss",
            &current_frame_second.to_string(),
            "-frames:v",
            "1",
            "-vf",
            &preview_scale_args,
            &preview_file_path_string,
        ])
        .status()
        .with_context(|| {
            format!(
                "generate_preview_by_current_frame: failed to spawn new command for {:?}",
                preview_file_path_string
            )
        })?;

    if !status.success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "ffmpeg failed to generate preview",
        )));
    }
    Ok(())
}