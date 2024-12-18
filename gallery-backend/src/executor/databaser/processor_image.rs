use std::{collections::BTreeMap, error::Error, io, mem, path::Path};

use anyhow::Context;
use image::DynamicImage;

use crate::public::database_struct::database::definition::DataBase;

use super::{
    fix_orientation::fix_orientation,
    generate_dynamic_image::generate_dynamic_image,
    generate_exif::generate_exif,
    generate_width_height::{generate_img_width_height, generate_phash, generate_thumbhash}, generate_compressed_image::generate_compressed_image,
};

pub fn process_image_info(database: &mut DataBase) -> Result<(), Box<dyn Error>> {
    database.exif_vec = generate_exif(&database);
    let mut dynamic_image = generate_dynamic_image(&database)?;
    (database.width, database.height) = generate_img_width_height(&dynamic_image);
    fix_orientation(database, &mut dynamic_image);
    database.thumbhash = generate_thumbhash(&dynamic_image)?;
    database.phash = generate_phash(&dynamic_image);
    generate_compressed_image(database, Some(dynamic_image))?;
    Ok(())
}
