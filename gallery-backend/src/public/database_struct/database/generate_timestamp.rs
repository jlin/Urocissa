use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc};
use rand::Rng;
use regex::Regex;

use std::{path::PathBuf, sync::LazyLock};

use super::definition::Database;

static FILE_NAME_TIME_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(\d{4})[^a-zA-Z0-9]?(\d{2})[^a-zA-Z0-9]?(\d{2})[^a-zA-Z0-9]?(\d{2})[^a-zA-Z0-9]?(\d{2})[^a-zA-Z0-9]?(\d{2})\b").unwrap()
});

impl Database {
    pub fn compute_timestamp(&self, priority_list: &[&str]) -> u128 {
        let now_time = Utc::now().naive_utc();
        for &field in priority_list {
            match field {
                "DateTimeOriginal" => {
                    if let Some(value) = self.exif_vec.get("DateTimeOriginal") {
                        if let Ok(date_time) =
                            NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S")
                        {
                            if date_time <= now_time {
                                return date_time.and_utc().timestamp_millis() as u128;
                            }
                        }
                    }
                }
                "filename" => {
                    if let Some(file_name_full_path) = self.alias.get(0) {
                        let path = PathBuf::from(&file_name_full_path.file);
                        if let Some(file_name) = path.file_name() {
                            if let Some(captures) =
                                &FILE_NAME_TIME_REGEX.captures(file_name.to_str().unwrap())
                            {
                                if let (
                                    Ok(year),
                                    Ok(month),
                                    Ok(day),
                                    Ok(hour),
                                    Ok(minute),
                                    Ok(second),
                                ) = (
                                    captures[1].parse::<i32>(),
                                    captures[2].parse::<u32>(),
                                    captures[3].parse::<u32>(),
                                    captures[4].parse::<u32>(),
                                    captures[5].parse::<u32>(),
                                    captures[6].parse::<u32>(),
                                ) {
                                    if let Some(data) = NaiveDate::from_ymd_opt(year, month, day) {
                                        if let Some(time) =
                                            NaiveTime::from_hms_opt(hour, minute, second)
                                        {
                                            let date = NaiveDateTime::new(data, time);
                                            return date.and_utc().timestamp_millis() as u128;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                "scan_time" => {
                    let latest_scan_time = self.alias.iter().map(|alias| alias.scan_time).max();
                    if let Some(latest_time) = latest_scan_time {
                        return latest_time as u128;
                    }
                }
                "modified" => {
                    // Find the alias with the maximum `scan_time`
                    if let Some(max_scan_alias) =
                        self.alias.iter().max_by_key(|alias| alias.scan_time)
                    {
                        return max_scan_alias.modified;
                    }
                }
                "random" => {
                    let mut rng = rand::rng();
                    let random_number: u128 = rng.random();
                    return random_number;
                }
                _ => panic!("Unknown field type: {}", field),
            }
        }
        0
    }
}
