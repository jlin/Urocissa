use super::small_width_height;
use crate::{
    indexer::databaser::generate_ffmpeg::create_silent_ffmpeg_command,
    public::structure::database_struct::database::definition::Database,
};
use anyhow::Context;
use image::{DynamicImage, ImageFormat};
use std::process::Stdio;
pub fn generate_thumbnail_for_image(
    database: &mut Database,
    dynamic_image: DynamicImage,
) -> anyhow::Result<()> {
    let (compressed_width, compressed_height) =
        small_width_height(database.width, database.height, 1280);

    let thumbnail_image = dynamic_image
        .thumbnail_exact(compressed_width, compressed_height)
        .to_rgb8();

    let binding = database.compressed_path();
    let parent_path = binding.parent().ok_or_else(|| {
        anyhow::anyhow!(
            "image_compressor: Failed to get parent directory for {:?}",
            database.compressed_path()
        )
    })?;

    std::fs::create_dir_all(parent_path)?;
    thumbnail_image.save_with_format(database.compressed_path(), ImageFormat::Jpeg)?;

    Ok(())
}

/// Generates a single JPEG thumbnail from the first frame of a video.
pub fn generate_thumbnail_for_video(database: &Database) -> anyhow::Result<()> {
    let (width, height) = (database.width, database.height);
    let (thumb_width, thumb_height) = small_width_height(width, height, 1280);
    let thumbnail_path = database.thumbnail_path();

    std::fs::create_dir_all(database.compressed_path_parent())
        .context("Failed to create parent directory for thumbnail")?;

    // --- REFACTORED: Use the helper for a fire-and-forget silent command ---
    let mut cmd = create_silent_ffmpeg_command();
    cmd.args([
        "-y",
        "-i",
        &database.imported_path_string(),
        "-ss",
        "0", // Seek to the beginning
        "-vframes",
        "1", // Extract exactly one frame
        "-vf",
        &format!("scale={}:{}", thumb_width, thumb_height),
        &thumbnail_path,
    ]);

    // For this command, we don't need to read any output, so we discard both streams.
    let status = cmd
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("Failed to execute ffmpeg for thumbnail generation")?;

    if !status.success() {
        return Err(anyhow::anyhow!(
            "ffmpeg thumbnail generation failed with exit code: {}",
            status.code().unwrap_or(-1)
        ));
    }

    Ok(())
}
