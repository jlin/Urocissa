use super::video_ffprobe::video_duration;
use crate::{
    indexer::databaser::processor::process_image_info,
    structure::database_struct::database::definition::Database,
};
use anyhow::Context;
use regex::Regex;
use std::{
    cmp,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

pub fn generate_compressed_video(database: &mut Database) -> anyhow::Result<()> {
    let duration_result = video_duration(&database.imported_path_string());

    let duration = match duration_result {
        Ok(d) if (d * 1000.0) as u32 == 100 => {
            // If duration is 0.1 seconds (0.1 seconds * 1000 microseconds/second = 100 microseconds)
            info!(
                "This gif is a static picture, try with image_compressor - {:?}",
                database.imported_path_string()
            );
            database.ext_type = "image".to_string();
            return process_image_info(database);
        }
        Ok(d) => d, // If no error and the duration is not 0.1 seconds, continue using this value
        Err(err)
            if err.to_string().contains("fail to parse to f32")
                && database.ext.eq_ignore_ascii_case("gif") =>
        {
            info!("This may not be a gif");
            database.ext_type = "image".to_string();
            return process_image_info(database);
        }
        Err(err) => {
            // Convert the Box<dyn Error> into anyhow::Error
            return Err(anyhow::anyhow!(
                "video_compressor: failed to get video duration for {:?}: {}",
                database.imported_path_string(),
                err
            ));
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
    let stdout = cmd.stdout.as_mut().ok_or_else(|| {
        anyhow::anyhow!(
            "video_compressor: failed to get stdout from ffmpeg command for {:?}",
            database.imported_path_string()
        )
    })?;
    let stdout_reader = BufReader::new(stdout);
    let stdout_lines = stdout_reader.lines();
    let stdout_lines_filtered = stdout_lines.filter_map(|line| {
        line.ok()
            .filter(|line_string| line_string.contains("out_time_us"))
    });
    let re = Regex::new(r"out_time_us=(\d+)").unwrap();
    for line in stdout_lines_filtered {
        if let Some(captured) = re.captures(&line)
            && let Some(processed_time) = captured.get(1)
        {
            match processed_time.as_str().parse::<f64>() {
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
    cmd.wait().unwrap();

    Ok(())
}
