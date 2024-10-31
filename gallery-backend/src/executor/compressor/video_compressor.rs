use crate::{
    executor::compressor::{image_compressor::image_compressor, video_ffprobe::video_width_height},
    public::{
        constant::SHOULD_SWAP_WIDTH_HEIGHT_ROTATION,
        database_struct::database::definition::DataBase,
    },
};
use anyhow::{Context, Result};
use regex::Regex;
use std::{
    cmp,
    error::Error,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

use super::{video_ffprobe::video_duration, video_preview::generate_preview};

pub fn video_compressor(database: &mut DataBase) -> Result<(), Box<dyn Error>> {
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
    std::fs::create_dir_all(database.compressed_path_parent()).with_context(|| {
        format!(
            "video_compressor: failed to create directory for {:?}",
            database.imported_path_string()
        )
    })?;
    generate_preview(database)?; // Get preview image
    database.pending = true; // Waiting to perform the next step (generate_compressed) in a worker thread

    Ok(())
}

pub fn generate_compressed(database: &mut DataBase) -> Result<(), Box<dyn Error>> {
    let duration_result = video_duration(&database.imported_path_string());

    let duration = match duration_result {
        Ok(d) if (d * 1000.0) as u32 == 100 => {
            // If duration is 0.1 seconds (0.1 seconds * 1000 microseconds/second = 100 microseconds)
            info!(
                "This gif is a static picture, try with image_compressor - {:?}",
                database.imported_path_string()
            );
            database.ext_type = "image".to_string();
            return image_compressor(database);
        }
        Ok(d) => d, // If no error and the duration is not 0.1 seconds, continue using this value
        Err(e) => {
            if e.to_string().contains("fail to parse to f32")
                && database.ext.to_lowercase() == String::from("gif")
            {
                info!("This may not be a gif");
                database.ext_type = "image".to_string();
                return image_compressor(database);
            } else {
                return Err(e);
            }
        }
    };
    let mut cmd = Command::new("ffmpeg")
        .args(&[
            "-y", // Add this line to allow automatic file overwrite
            "-i",
            &database.imported_path_string(),
            "-vf",
            &format!(
                "scale=trunc((oh*a)/2)*2:{}",
                (cmp::min(database.height, 720) / 2) * 2
            ),
            "-movflags",
            "faststart",
            &database.compressed_path_string(),
            "-progress",
            "pipe:1",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .with_context(|| {
            format!(
                "video_compressor: failed to spawn new command for ffmpeg: {:?}",
                database.imported_path_string()
            )
        })?;
    let stdout = cmd.stdout.as_mut().ok_or("Failed to get command output")?;
    let stdout_reader = BufReader::new(stdout);
    let stdout_lines = stdout_reader.lines();
    let stdout_lines_filtered = stdout_lines.filter_map(|line| {
        line.ok()
            .filter(|line_string| line_string.contains("out_time_us"))
    });
    let re = Regex::new(r"out_time_us=(\d+)").unwrap();
    for line in stdout_lines_filtered {
        if let Some(captured) = re.captures(&line) {
            if let Some(processed_time) = captured.get(1) {
                let processed_time_str = processed_time.as_str();
                match processed_time_str.parse::<f64>() {
                    Ok(processed_time_f64) => {
                        // Microseconds
                        let x = ((processed_time_f64 / 1000000.0) / duration) * 100.0;
                        info!(
                            "Percentage: {:.2}% for {}",
                            x,
                            &database.compressed_path_string()
                        );
                    }
                    Err(e) => error!("Failed to parse processed_time: {}", e),
                }
            } else {
                error!("No digits captured for line: {}", line);
            }
        }
    }
    cmd.wait().unwrap();

    Ok(())
}
