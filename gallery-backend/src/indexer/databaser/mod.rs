use self::processor::{process_image_info, process_video_info};
use crate::constant::VALID_IMAGE_EXTENSIONS;
use crate::constant::redb::DATA_TABLE;
use crate::coordinator::delete::DeleteTask;
use crate::coordinator::video::VideoTask;
use crate::coordinator::{COORDINATOR, Task};
use crate::db::tree::TREE;
use crate::structure::database_struct::database::definition::Database;
use std::cmp;

use std::path::PathBuf;
pub mod fix_orientation;
pub mod generate_compressed_video;
pub mod generate_dynamic_image;
pub mod generate_exif;
pub mod generate_image_hash;
pub mod generate_thumbnail;
pub mod generate_width_height;
pub mod processor;
pub mod video_ffprobe;
pub fn databaser(mut database: Database) -> anyhow::Result<()> {
    let write_txn = TREE.in_disk.begin_write().unwrap();
    let is_image = VALID_IMAGE_EXTENSIONS.contains(&database.ext.as_str());
    {
        let mut write_table = write_txn.open_table(DATA_TABLE).unwrap();

        if is_image {
            process_image_info(&mut database)?;
        } else {
            process_video_info(&mut database)?;

            database.pending = true;
        }

        write_table
            .insert(&*database.hash, database.clone())
            .unwrap();

        if let Some(latest) = database.alias.iter().max_by_key(|a| a.scan_time) {
            COORDINATOR.submit(Task::Delete(DeleteTask::new(PathBuf::from(&latest.file))))?
        };
    }

    write_txn.commit().unwrap();
    if !is_image {
        COORDINATOR.submit(Task::Video(VideoTask::new(database.hash)))?;
    }

    Ok(())
}

pub fn small_width_height(width: u32, height: u32, small_height: u32) -> (u32, u32) {
    let (nwidth, nheight) = if width >= cmp::max(height, small_height) {
        (small_height, height * small_height / width)
    } else if height >= cmp::max(width, small_height) {
        (width * small_height / height, small_height)
    } else {
        (width, height)
    };
    return (nwidth, nheight);
}
