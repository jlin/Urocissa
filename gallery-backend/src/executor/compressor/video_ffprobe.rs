use std::error::Error;
use std::process::Command;

use anyhow::Context;
pub fn video_width_height(info: &str, file_path: &str) -> Result<u32, Box<dyn Error>> {
    let command_text = match info {
        "width" => Ok("stream=width"),
        "height" => Ok("stream=height"),
        _ => Err(anyhow::Error::msg(
            "video_duration_width_height: command error",
        )),
    };
    let output = Command::new("ffprobe")
        .args(&[
            "-v",
            "error",
            "-show_entries",
            command_text?,
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            file_path,
        ])
        .output()
        .with_context(|| {
            format!(
                "video_duration_width_height: fail to spawn new command for ffmpeg: {:?}",
                file_path
            )
        })?;
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?.trim().parse::<u32>()?)
    } else {
        Err(From::from(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }
}

pub fn video_duration(file_path: &str) -> Result<f64, Box<dyn Error>> {
    let output = Command::new("ffprobe")
        .args(&[
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            file_path,
        ])
        .output()
        .with_context(|| {
            format!(
                "video_duration_width_height: fail to spawn new command for ffmpeg: {:?}",
                file_path
            )
        })?;
    if output.status.success() {
        let duration_in_seconds = String::from_utf8(output.stdout)?
            .trim()
            .parse::<f64>()
            .with_context(|| {
                format!(
                    "video_duration_width_height: fail to parse to f64: {:?}",
                    file_path
                )
            })?;
        Ok(duration_in_seconds)
    } else {
        Err(From::from(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }
}
