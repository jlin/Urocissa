use super::video_ffprobe::video_duration;
use crate::{
    indexer::databaser::process_image_info,
    structure::database_struct::database::definition::Database,
};
use anyhow::Context;
use regex::Regex;
use std::{
    cmp,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    sync::LazyLock,
};

static REGEX_OUT_TIME_US: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"out_time_us=(\d+)").unwrap());

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
    // QUIET but still emits machine-readable progress --------------------
    let mut child = Command::new("ffmpeg")
        .args([
            // Silence absolutely everything
            "-v",
            "quiet", // no banner, no info, no warnings
            "-hide_banner",
            "-nostats", // hide per-second counter
            "-nostdin", // suppress “[q]” prompt
            "-y",
            // input/output...
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
            // progress now goes to stderr (fd 2)
            "-progress",
            "pipe:2",
        ])
        // we parse *stderr*; stdout is sent to /dev/null
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .context("failed to spawn ffmpeg")?;

    let stderr = child.stderr.take().unwrap();
    let reader = BufReader::new(stderr);

    for line in reader.lines().flatten() {
        if let Some(caps) = REGEX_OUT_TIME_US.captures(&line) {
            if let Ok(us) = caps[1].parse::<f64>() {
                let pct = (us / 1_000_000.0) / duration * 100.0;
                info!(
                    "Percentage: {pct:.2}% for {}",
                    database.compressed_path_string()
                );
            }
        }
    }
    child.wait()?; // remember to await the child

    Ok(())
}
