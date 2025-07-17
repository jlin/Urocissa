use crate::public::structure::database_struct::database::definition::Database;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;
use image::{DynamicImage, ImageBuffer, Rgb};
use std::fs::read;
use std::path::PathBuf;
use zune_jpeg::{JpegDecoder, zune_core::options::DecoderOptions};
pub fn generate_dynamic_image(database: &Database) -> Result<DynamicImage> {
    let img_path = if database.ext_type == "image".to_string() {
        database.imported_path()
    } else {
        PathBuf::from(database.thumbnail_path())
    };
    let dynamic_image = decode_image(&img_path)?;
    Ok(dynamic_image)
}

fn decode_image(file_path: &PathBuf) -> Result<DynamicImage> {
    let file_in_memory = read(file_path)?;

    let is_jpeg = file_path.extension().map_or(false, |ext| {
        ["jpg", "jpeg"].contains(&ext.to_string_lossy().to_lowercase().as_ref())
    });

    let decoders: Vec<fn(&Vec<u8>) -> Result<DynamicImage>> = if is_jpeg {
        vec![image_crate_decoder, zune_jpeg_decoder]
    } else {
        vec![image_crate_decoder]
    };

    for decoder in decoders {
        match decoder(&file_in_memory) {
            Ok(decoded_image) => return Ok(decoded_image),
            Err(_) => continue,
        }
    }

    bail!("All decoders failed");
}

fn image_crate_decoder(file_in_memory: &Vec<u8>) -> Result<DynamicImage> {
    let dynamic_image = image::load_from_memory(file_in_memory)?;
    Ok(dynamic_image)
}

fn zune_jpeg_decoder(file_in_memory: &Vec<u8>) -> Result<DynamicImage> {
    let mut decoder = JpegDecoder::new(file_in_memory);
    decoder.set_options(DecoderOptions::new_fast());
    let pixels = decoder.decode()?;

    let info = decoder
        .info()
        .ok_or_else(|| anyhow!("Failed to retrieve JPEG info"))?;
    let width = info.width as u32;
    let height = info.height as u32;
    let buffer = ImageBuffer::<Rgb<u8>, _>::from_raw(width, height, pixels)
        .ok_or_else(|| anyhow!("Failed to create image buffer from raw data"))?;

    let dynamic_image = DynamicImage::ImageRgb8(buffer);
    Ok(dynamic_image)
}
