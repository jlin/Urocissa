use crate::executor::compressor::image_decoder;
use crate::public::database_struct::database::definition::DataBase;

use image::DynamicImage;
use std::error::Error;
use std::path::PathBuf;
pub fn generate_dynamic_image(database: &DataBase) -> Result<DynamicImage, Box<dyn Error>> {
    let img_path = if database.ext_type == "image".to_string() {
        database.imported_path()
    } else {
        PathBuf::from(database.preview_path())
    };
    let dynamic_image = image_decoder::decoder_image(&img_path)?;
    Ok(dynamic_image)
}
