use crate::public::database_struct::database::definition::DataBase;

use image::DynamicImage;
use std::error::Error;
use std::path::PathBuf;

use super::image_decoder::decode_image;
pub fn generate_dynamic_image(database: &DataBase) -> Result<DynamicImage, Box<dyn Error>> {
    let img_path = if database.ext_type == "image".to_string() {
        database.imported_path()
    } else {
        PathBuf::from(database.preview_path())
    };
    let dynamic_image = decode_image(&img_path)?;
    Ok(dynamic_image)
}
