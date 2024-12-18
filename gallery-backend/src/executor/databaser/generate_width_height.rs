use crate::public::database_struct::database::definition::DataBase;

use super::video_ffprobe::video_width_height;
use image::DynamicImage;
use std::error::Error;

pub fn generate_image_width_height(dynamic_image: &DynamicImage) -> (u32, u32) {
    let width = dynamic_image.width();
    let height = dynamic_image.height();
    (width, height)
}

pub fn generate_video_width_height(database: &DataBase) -> Result<(u32, u32), Box<dyn Error>> {
    let width = video_width_height("width", &database.imported_path_string())?;
    let height = video_width_height("height", &database.imported_path_string())?;
    Ok((width, height))
}
