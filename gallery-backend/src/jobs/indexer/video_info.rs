use std::fs::metadata;

use crate::operations::indexer::fix_orientation::fix_video_width_height;
use crate::operations::indexer::generate_dynamic_image::generate_dynamic_image;
use crate::operations::indexer::generate_exif::generate_exif_for_video;
use crate::operations::indexer::generate_image_hash::{
    generate_phash, generate_thumbhash,
};
use crate::operations::indexer::generate_thumbnail::generate_thumbnail_for_video;
use crate::operations::indexer::generate_width_height::generate_video_width_height;
use crate::public::structure::database_struct::database::definition::Database;

pub fn process_video_info(database: &mut Database) -> anyhow::Result<()> {
    database.exif_vec = generate_exif_for_video(&database)?;
    (database.width, database.height) = generate_video_width_height(&database)?;
    fix_video_width_height(database);
    generate_thumbnail_for_video(database)?;
    let dynamic_image = generate_dynamic_image(database)?;
    database.thumbhash = generate_thumbhash(&dynamic_image)?;
    database.phash = generate_phash(&dynamic_image);
    Ok(())
}

pub fn regenerate_metadata_for_video(database: &mut Database) -> anyhow::Result<()> {
    database.size = metadata(&database.imported_path())?.len();
    process_video_info(database)?;
    Ok(())
}
