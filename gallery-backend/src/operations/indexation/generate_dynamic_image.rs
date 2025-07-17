use crate::public::structure::database_struct::database::definition::Database;
use anyhow::{Result, Context, anyhow, bail};
use image::{DynamicImage, ImageBuffer, Rgb};
use std::fs::read;
use std::path::PathBuf;
use zune_jpeg::{JpegDecoder, zune_core::options::DecoderOptions};

/// Generate a `DynamicImage` either from the original image or
/// from its thumbnail, adding *context* at every fallible step.
pub fn generate_dynamic_image(database: &Database) -> Result<DynamicImage> {
    let img_path = if database.ext_type == "image" {
        database.imported_path()
    } else {
        PathBuf::from(database.thumbnail_path())
    };

    let dynamic_image = decode_image(&img_path)
        .context(format!("failed to decode image: {:?}", img_path))?;

    Ok(dynamic_image)
}

fn decode_image(file_path: &PathBuf) -> Result<DynamicImage> {
    let file_in_memory = read(file_path)
        .context(format!("failed to read file into memory: {:?}", file_path))?;

    let is_jpeg = file_path
        .extension()
        .map_or(false, |ext| ["jpg", "jpeg"].contains(&ext.to_string_lossy().to_lowercase().as_ref()));

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

    bail!("all decoders failed for file: {:?}", file_path);
}

fn image_crate_decoder(file_in_memory: &Vec<u8>) -> Result<DynamicImage> {
    let dynamic_image = image::load_from_memory(file_in_memory)
        .context("image crate failed to decode image from memory")?;
    Ok(dynamic_image)
}

fn zune_jpeg_decoder(file_in_memory: &Vec<u8>) -> Result<DynamicImage> {
    let mut decoder = JpegDecoder::new(file_in_memory);
    decoder.set_options(DecoderOptions::new_fast());

    let pixels = decoder
        .decode()
        .context("zune-jpeg failed to decode JPEG data")?;

    let info = decoder
        .info()
        .ok_or_else(|| anyhow!("failed to retrieve JPEG info"))?;
    let width = info.width as u32;
    let height = info.height as u32;

    let buffer = ImageBuffer::<Rgb<u8>, _>::from_raw(width, height, pixels)
        .ok_or_else(|| anyhow!("failed to create image buffer from raw data"))?;

    Ok(DynamicImage::ImageRgb8(buffer))
}