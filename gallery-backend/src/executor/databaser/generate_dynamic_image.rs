use crate::public::database_struct::database::definition::DataBase;

use image::DynamicImage;
use std::error::Error;
use std::path::PathBuf;

use super::generate_width_height::generate_thumbhash;
use super::image_decoder::decoder_image;
pub fn generate_dynamic_image(database: &DataBase) -> Result<DynamicImage, Box<dyn Error>> {
    let img_path = if database.ext_type == "image".to_string() {
        database.imported_path()
    } else {
        PathBuf::from(database.preview_path())
    };
    let dynamic_image = decoder_image(&img_path)?;
    Ok(dynamic_image)
}
