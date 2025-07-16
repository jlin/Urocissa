use std::cmp;

pub mod fix_orientation;
pub mod generate_compressed_video;
pub mod generate_dynamic_image;
pub mod generate_exif;
pub mod generate_ffmpeg;
pub mod generate_image_hash;
pub mod generate_thumbnail;
pub mod generate_width_height;
pub mod video_ffprobe;

use crate::operations;
use crate::public::structure::database_struct::database::definition::Database;
use crate::public::tui::{DASHBOARD, FileType};
use crate::tasks::looper::LOOPER;
use crate::tasks::looper::Signal;

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
