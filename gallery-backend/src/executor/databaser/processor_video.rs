use crate::public::database_struct::database::definition::DataBase;

use std::error::Error;

use super::fix_orientation::fix_video_orientation;
use super::generate_dynamic_image::generate_dynamic_image;
use super::generate_exif::generate_video_exif;
use super::generate_image_hash::generate_thumbhash;
use super::generate_preview::generate_preview;
use super::generate_width_height::generate_video_width_height;

pub fn process_video_info(database: &mut DataBase) -> Result<(), Box<dyn Error>> {
    database.exif_vec = generate_video_exif(database.source_path_string())?;
    generate_preview(database)?;
    let dynamic_image = generate_dynamic_image(database)?;
    database.thumbhash = generate_thumbhash(&dynamic_image)?;
    (database.width, database.height) = generate_video_width_height(&database)?;
    fix_video_orientation(database);
    Ok(())
}
