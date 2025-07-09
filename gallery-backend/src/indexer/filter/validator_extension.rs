use crate::constant::{VALID_IMAGE_EXTENSIONS, VALID_VIDEO_EXTENSIONS};
use anyhow::bail;
use std::{ffi::OsStr, path::PathBuf};

pub fn validator(path: &PathBuf) -> anyhow::Result<()> {
    let extension = path.extension().and_then(OsStr::to_str);

    match extension {
        Some(ext) => {
            let lowercased_ext = ext.to_ascii_lowercase();
            if VALID_IMAGE_EXTENSIONS.contains(&lowercased_ext.as_str())
                || VALID_VIDEO_EXTENSIONS.contains(&lowercased_ext.as_str())
            {
                Ok(())
            } else {
                bail!("{} is not a valid extension", &lowercased_ext);
            }
        }
        None => {
            bail!("{} has no extension", path.display());
        }
    }
}
