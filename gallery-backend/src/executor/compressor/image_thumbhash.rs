use crate::public::database_struct::database::definition::DataBase;

use super::image_decoder;
use image::DynamicImage;
use image_hasher::HasherConfig;
use std::error::Error;
use std::path::PathBuf;
pub fn generate_thumbhash(
    database: &mut DataBase,
    img_path: &PathBuf,
) -> Result<(u32, u32, DynamicImage), Box<dyn Error>> {
    let dynamic_image = image_decoder::decoder_image(img_path)?;
    let dynamic_image_rotated = handle_orientation(database, dynamic_image);
    let width = dynamic_image_rotated.width();
    let height = dynamic_image_rotated.height();
    let resized_image = dynamic_image_rotated.thumbnail_exact(100, 100);
    let rgba_image = resized_image.to_rgba8();
    let (swidth, sheight) = (rgba_image.width(), rgba_image.height());
    let thumbhash = thumbhash::rgba_to_thumb_hash(swidth as usize, sheight as usize, &rgba_image);
    let hasher = HasherConfig::new().to_hasher();
    let phash = hasher.hash_image(&dynamic_image_rotated);
    database.width = width;
    database.height = height;
    database.thumbhash = thumbhash;
    database.phash = phash.as_bytes().to_vec();
    Ok((width, height, dynamic_image_rotated))
}

fn handle_orientation(database: &mut DataBase, mut dynamic_image: DynamicImage) -> DynamicImage {
    if let Some(orientation) = database.exif_vec.get("Orientation") {
        match orientation.as_str() {
            "row 0 at right and column 0 at top" => {
                dynamic_image = dynamic_image.rotate90();
                std::mem::swap(&mut database.width, &mut database.height)
            }
            "row 0 at bottom and column 0 at right" => dynamic_image = dynamic_image.rotate180(),
            "row 0 at left and column 0 at bottom" => {
                dynamic_image = dynamic_image.rotate270();
                std::mem::swap(&mut database.width, &mut database.height)
            }
            _ => (),
        }
    }
    return dynamic_image;
}
