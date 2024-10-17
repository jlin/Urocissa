use crate::public::database_struct::database::definition::DataBase;
use crate::public::database_struct::hash_alias::HashAliasSize;
use anyhow::Context;
use regex::Regex;
use std::collections::BTreeMap;
use std::error::Error;
use std::process::Command;
use std::sync::LazyLock;

static RE_VIDEO_INFO: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(.*?)=(.*?)\n").unwrap());

pub fn process_video_info(hash_alias_size: &mut HashAliasSize) -> Result<DataBase, Box<dyn Error>> {
    let mut exif_tuple = BTreeMap::new();
    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("error")
        .arg("-show_format")
        .arg("-show_streams")
        .arg(&hash_alias_size.hash_alias.source_path())
        .output()
        .with_context(|| {
            format!(
                "process_video_info: spawn new command for ffprobe failed for {:?}",
                hash_alias_size.hash_alias.source_path()
            )
        })?;

    if output.status.success() {
        let line = String::from_utf8(output.stdout).with_context(|| {
            format!(
                "process_video_info: Failed to from vec<u8> to String for {:?}",
                hash_alias_size.hash_alias.source_path()
            )
        })?;
        for mat in RE_VIDEO_INFO.captures_iter(&line) {
            let key = mat
                .get(1)
                .with_context(|| {
                    format!(
                        "process_video_info: Failed to get(1) of match {:?} for {:?}",
                        mat,
                        hash_alias_size.hash_alias.source_path()
                    )
                })?
                .as_str()
                .to_string();
            let value = mat
                .get(2)
                .with_context(|| {
                    format!(
                        "process_video_info: Failed to get(2) of match {:?} for {:?}",
                        mat,
                        hash_alias_size.hash_alias.source_path()
                    )
                })?
                .as_str()
                .to_string();
            exif_tuple.insert(key, value);
        }
        Ok(DataBase::new(
            std::mem::take(&mut hash_alias_size.hash_alias.hash),
            hash_alias_size.size,
            hash_alias_size.hash_alias.ext(),
            std::mem::take(&mut exif_tuple),
            std::mem::take(&mut hash_alias_size.hash_alias.alias.alias),
        ))
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Command execution failed",
        )))
    }
}
