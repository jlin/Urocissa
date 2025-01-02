use std::{collections::BTreeMap, error::Error, io, path::Path, process::Command, sync::LazyLock};

use anyhow::Context;
use regex::Regex;

use crate::public::{
    constant::VALID_IMAGE_EXTENSIONS, database_struct::database::definition::DataBase,
};

pub fn generate_image_exif(database: &DataBase) -> BTreeMap<String, String> {
    extract_image_exif(&database.source_path())
}

pub fn regenerate_exif(database: &DataBase) -> BTreeMap<String, String> {
    if VALID_IMAGE_EXTENSIONS.contains(&database.ext.as_str()) {
        extract_image_exif(&database.imported_path())
    } else {
        generate_video_exif(&database.imported_path_string()).unwrap()
    }
}

fn extract_image_exif(path: &Path) -> BTreeMap<String, String> {
    let mut exif_tuple = BTreeMap::new();
    if let Ok(exif) = read_exif(path) {
        for field in exif.fields() {
            let tag = field.tag.to_string();
            let value = field.display_value().with_unit(&exif).to_string();
            let ifd_num = field.ifd_num;
            if exif_tuple.get(&tag).is_some() {
                if ifd_num == exif::In::PRIMARY {
                    exif_tuple.insert(tag, value);
                }
            } else {
                exif_tuple.insert(tag, value);
            }
        }
    }
    exif_tuple
}

fn read_exif(file_path: &Path) -> Result<exif::Exif, Box<dyn Error>> {
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

pub fn generate_video_exif(source_path: &str) -> Result<BTreeMap<String, String>, Box<dyn Error>> {
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
