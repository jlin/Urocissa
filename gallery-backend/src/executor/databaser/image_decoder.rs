use image::{DynamicImage, ImageBuffer, Rgb};
use std::{error::Error, fs::read, path::PathBuf};
use zune_jpeg::{zune_core::options::DecoderOptions, JpegDecoder};
pub fn decode_image(file_path: &PathBuf) -> Result<DynamicImage, Box<dyn Error>> {
    let file_in_memory = read(file_path)?;

    let is_jpeg = file_path.extension().map_or(false, |ext| {
        ["jpg", "jpeg"].contains(&ext.to_string_lossy().to_lowercase().as_ref())
    });

    let decoders: Vec<fn(&Vec<u8>) -> Result<DynamicImage, Box<dyn Error>>> = if is_jpeg {
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

    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "All decoders failed",
    )))
}

pub fn image_crate_decoder(file_in_memory: &Vec<u8>) -> Result<DynamicImage, Box<dyn Error>> {
    let dynamic_image = image::load_from_memory(file_in_memory)?;
    Ok(dynamic_image)
}

pub fn zune_jpeg_decoder(file_in_memory: &Vec<u8>) -> Result<DynamicImage, Box<dyn Error>> {
    let mut decoder = JpegDecoder::new(file_in_memory);
    decoder.set_options(DecoderOptions::new_fast());
    let pixels = decoder.decode()?;

    let info = decoder.info().ok_or("Failed to retrieve JPEG info")?;
    let width = info.width as u32;
    let height = info.height as u32;
    let buffer = ImageBuffer::<Rgb<u8>, _>::from_raw(width, height, pixels)
        .ok_or("Failed to create image buffer from raw data")?;

    let dynamic_image = DynamicImage::ImageRgb8(buffer);
    Ok(dynamic_image)
}
