use super::video_ffprobe::video_width_height;
use crate::structure::database_struct::database::definition::Database;
use anyhow::Result;
use image::DynamicImage;
pub fn generate_image_width_height(dynamic_image: &DynamicImage) -> (u32, u32) {
    let width = dynamic_image.width();
    let height = dynamic_image.height();
    (width, height)
}

pub fn generate_video_width_height(database: &Database) -> Result<(u32, u32)> {
    let width = video_width_height("width", &database.imported_path_string())?;
    let height = video_width_height("height", &database.imported_path_string())?;
    Ok((width, height))
}
