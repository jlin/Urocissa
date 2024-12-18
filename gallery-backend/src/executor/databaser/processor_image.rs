use std::error::Error;

use crate::public::database_struct::database::definition::DataBase;

use super::{
    fix_orientation::{fix_image_orientation, fix_image_width_height, fix_video_width_height},
    generate_compressed_image::generate_compressed_image,
    generate_dynamic_image::generate_dynamic_image,
    generate_exif::{generate_image_exif, generate_video_exif},
    generate_image_hash::{generate_phash, generate_thumbhash},
    generate_preview::generate_preview,
    generate_width_height::{generate_image_width_height, generate_video_width_height},
};

pub fn process_image_info(database: &mut DataBase) -> Result<(), Box<dyn Error>> {
    database.exif_vec = generate_image_exif(&database);
    let mut dynamic_image = generate_dynamic_image(&database)?;
    (database.width, database.height) = generate_image_width_height(&dynamic_image);
    fix_image_width_height(database);
    fix_image_orientation(database, &mut dynamic_image);
    database.thumbhash = generate_thumbhash(&dynamic_image)?;
    database.phash = generate_phash(&dynamic_image);
    generate_compressed_image(database, Some(dynamic_image))?;
    Ok(())
}

pub fn process_video_info(database: &mut DataBase) -> Result<(), Box<dyn Error>> {
    database.exif_vec = generate_video_exif(database.source_path_string())?;
    (database.width, database.height) = generate_video_width_height(&database)?;
    fix_video_width_height(database);
    generate_preview(database)?;
    let dynamic_image = generate_dynamic_image(database)?;
    database.thumbhash = generate_thumbhash(&dynamic_image)?;
    database.phash = generate_phash(&dynamic_image);
    Ok(())
}
