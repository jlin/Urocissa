use crate::executor::compressor::image_decoder;
use crate::executor::compressor::video_ffprobe::video_width_height;
use crate::public::database_struct::database::definition::DataBase;

use image::DynamicImage;
use image_hasher::HasherConfig;
use std::error::Error;
use std::path::PathBuf;

use super::generate_dynamic_image::generate_dynamic_image;

pub fn generate_img_width_height(dynamic_image: &DynamicImage) -> (u32, u32) {
    let width = dynamic_image.width();
    let height = dynamic_image.height();
    (width, height)
}

pub fn generate_video_width_height(database: &DataBase) -> Result<(u32, u32), Box<dyn Error>> {
    let width = video_width_height("width", &database.imported_path_string())?;
    let height = video_width_height("height", &database.imported_path_string())?;
    Ok((width, height))
}

pub fn generate_thumbhash(dynamic_image_rotated: &DynamicImage) -> Result<Vec<u8>, Box<dyn Error>> {
    let resized_image = dynamic_image_rotated.thumbnail_exact(100, 100);
    let rgba_image = resized_image.to_rgba8();
    let (swidth, sheight) = (rgba_image.width(), rgba_image.height());
    let thumbhash = thumbhash::rgba_to_thumb_hash(swidth as usize, sheight as usize, &rgba_image);
    Ok(thumbhash)
}

pub fn generate_phash(dynamic_image_rotated: &DynamicImage) -> Vec<u8> {
    let hasher = HasherConfig::new().to_hasher();
    let phash = hasher.hash_image(dynamic_image_rotated);
    phash.as_bytes().to_vec()
}
