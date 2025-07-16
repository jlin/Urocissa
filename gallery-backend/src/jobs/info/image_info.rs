use std::fs::metadata;

use crate::operations::indexation::fix_orientation::{
    fix_image_orientation, fix_image_width_height,
};
use crate::operations::indexation::generate_dynamic_image::generate_dynamic_image;
use crate::operations::indexation::generate_exif::generate_exif_for_image;
use crate::operations::indexation::generate_image_hash::{
    generate_phash, generate_thumbhash,
};
use crate::operations::indexation::generate_thumbnail::generate_thumbnail_for_image;
use crate::operations::indexation::generate_width_height::generate_image_width_height;
use crate::public::structure::database_struct::database::definition::Database;

pub fn process_image_info(database: &mut Database) -> anyhow::Result<()> {
    database.exif_vec = generate_exif_for_image(&database);
    let mut dynamic_image = generate_dynamic_image(&database)?;
    (database.width, database.height) = generate_image_width_height(&dynamic_image);
    fix_image_width_height(database);
    fix_image_orientation(database, &mut dynamic_image);
    database.thumbhash = generate_thumbhash(&dynamic_image)?;
    database.phash = generate_phash(&dynamic_image);
    generate_thumbnail_for_image(database, dynamic_image)?;
    Ok(())
}

pub fn regenerate_metadata_for_image(database: &mut Database) -> anyhow::Result<()> {
    database.size = metadata(&database.imported_path())?.len();
    process_image_info(database)?;
    Ok(())
}
