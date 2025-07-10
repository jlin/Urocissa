use crate::structure::database_struct::database::definition::Database;
use anyhow::Context;
use regex::Regex;
use std::{collections::BTreeMap, error::Error, io, path::Path, process::Command, sync::LazyLock};
pub fn generate_exif_for_image(database: &Database) -> BTreeMap<String, String> {
    let mut exif_tuple = BTreeMap::new();
    if let Ok(exif) = read_exif(&database.source_path()) {
        for field in exif.fields() {
            if field.ifd_num == exif::In::PRIMARY {
                let tag = field.tag.to_string();
                let value = field.display_value().with_unit(&exif).to_string();
                exif_tuple.insert(tag, value);
            }
        }
    }
    exif_tuple
}

fn read_exif(file_path: &Path) -> anyhow::Result<exif::Exif, Box<dyn Error>> {
    let exif_reader = exif::Reader::new();
    let file = std::fs::File::open(file_path)
        .with_context(|| format!("read_exif: Failed to open file {:?}", file_path))?;
    let mut bufreader = io::BufReader::with_capacity(1024 * 1024, &file);
    let exif = exif_reader
        .read_from_container(&mut bufreader)
        .with_context(|| format!("read_exif: Failed to read exif of file {:?}", file_path))?;
    Ok(exif)
}

static RE_VIDEO_INFO: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(.*?)=(.*?)\n").unwrap());

pub fn generate_exif_for_video(database: &Database) -> anyhow::Result<BTreeMap<String, String>> {
    let source_path = database.source_path_string();
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
        Err(anyhow::anyhow!(
            "process_video_info: ffprobe command failed with exit code {:?}",
            output.status.code().unwrap_or(-1)
        ))
    }
}
