use std::path::PathBuf;

use crate::jobs::indexer::image_info::process_image_info;
use crate::jobs::indexer::video_info::process_video_info;
use crate::public::constant::VALID_IMAGE_EXTENSIONS;
use crate::tasks::actor::delete::DeleteTask;
use crate::tasks::actor::video::VideoTask;
use crate::tasks::batcher::flush_tree::FLUSH_TREE_QUEUE;

use crate::public::structure::database_struct::database::definition::Database;
use crate::tasks::COORDINATOR;

pub mod image_info;
pub mod video_info;
