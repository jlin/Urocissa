use crate::public::constant::SHOULD_SWAP_WIDTH_HEIGHT_ROTATION;
use crate::public::database_struct::database::definition::DataBase;
use anyhow::Context;
use regex::Regex;
use std::collections::BTreeMap;
use std::error::Error;
use std::mem;
use std::process::Command;
use std::sync::LazyLock;

use super::generate_dynamic_image::generate_dynamic_image;
use super::generate_preview::generate_preview;
use super::generate_width_height::generate_thumbhash;
use super::video_ffprobe::video_width_height;

static RE_VIDEO_INFO: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(.*?)=(.*?)\n").unwrap());

pub fn process_video_info(database: &mut DataBase) -> Result<(), Box<dyn Error>> {
    database.exif_vec = generate_exif(database.source_path_string())?;
    generate_preview(database)?;
    let dynamic_image = generate_dynamic_image(database)?;
    database.thumbhash = generate_thumbhash(&dynamic_image)?;
    
    let mut width = video_width_height("width", &database.imported_path_string())?;
    let mut height = video_width_height("height", &database.imported_path_string())?;
    let should_swap_video_width_height = {
        if let Some(rotation) = database.exif_vec.get("rotation") {
            SHOULD_SWAP_WIDTH_HEIGHT_ROTATION.contains(&rotation.trim())
        } else {
            false
        }
    };
    if should_swap_video_width_height {
        (width, height) = (height, width)
    }
    database.width = width;
    database.height = height;

    
    Ok(())
}

pub fn generate_exif(source_path: &str) -> Result<BTreeMap<String, String>, Box<dyn Error>> {
    let mut exif_tuple = BTreeMap::new();
    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("error")
        .arg("-show_format")
        .arg("-show_streams")
        .arg(source_path)
        .output()
        .with_context(|| {
            format!(
                "process_video_info: spawn new command for ffprobe failed for {:?}",
                source_path
            )
        })?;

    if output.status.success() {
        let line = String::from_utf8(output.stdout).with_context(|| {
            format!(
                "process_video_info: Failed to from vec<u8> to String for {:?}",
                source_path
            )
        })?;
        for mat in RE_VIDEO_INFO.captures_iter(&line) {
            let key = mat
                .get(1)
                .with_context(|| {
                    format!(
                        "process_video_info: Failed to get(1) of match {:?} for {:?}",
                        mat, source_path
                    )
                })?
                .as_str()
                .to_string();
            let value = mat
                .get(2)
                .with_context(|| {
                    format!(
                        "process_video_info: Failed to get(2) of match {:?} for {:?}",
                        mat, source_path
                    )
                })?
                .as_str()
                .to_string();
            exif_tuple.insert(key, value);
        }
        Ok(exif_tuple)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Command execution failed",
        )))
    }
}
