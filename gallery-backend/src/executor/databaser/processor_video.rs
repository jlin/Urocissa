use crate::public::database_struct::database::definition::DataBase;
use anyhow::Context;
use regex::Regex;
use std::collections::BTreeMap;
use std::error::Error;
use std::mem;
use std::process::Command;
use std::sync::LazyLock;

static RE_VIDEO_INFO: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(.*?)=(.*?)\n").unwrap());

pub fn process_video_info(database: &mut DataBase) -> Result<DataBase, Box<dyn Error>> {
    let mut exif_tuple = BTreeMap::new();
    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("error")
        .arg("-show_format")
        .arg("-show_streams")
        .arg(&database.alias[0].file)
        .output()
        .with_context(|| {
            format!(
                "process_video_info: spawn new command for ffprobe failed for {:?}",
                &database.alias[0].file
            )
        })?;

    if output.status.success() {
        let line = String::from_utf8(output.stdout).with_context(|| {
            format!(
                "process_video_info: Failed to from vec<u8> to String for {:?}",
                &database.alias[0].file
            )
        })?;
        for mat in RE_VIDEO_INFO.captures_iter(&line) {
            let key = mat
                .get(1)
                .with_context(|| {
                    format!(
                        "process_video_info: Failed to get(1) of match {:?} for {:?}",
                        mat, &database.alias[0].file
                    )
                })?
                .as_str()
                .to_string();
            let value = mat
                .get(2)
                .with_context(|| {
                    format!(
                        "process_video_info: Failed to get(2) of match {:?} for {:?}",
                        mat, &database.alias[0].file
                    )
                })?
                .as_str()
                .to_string();
            exif_tuple.insert(key, value);
        }
        database.exif_vec = exif_tuple;
        return Ok(mem::take(database));
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Command execution failed",
        )))
    }
}
