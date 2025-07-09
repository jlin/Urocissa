use anyhow::bail;
use rocket::{fs::TempFile, Route};

pub mod authenticate;
pub mod create_album;
pub mod create_share;
pub mod post_upload;

pub fn generate_post_routes() -> Vec<Route> {
    routes![
        authenticate::authenticate,
        create_album::create_non_empty_album,
        create_album::create_empty_album,
        post_upload::upload,
        create_share::create_share
    ]
}

pub fn get_extension(file: &TempFile<'_>) -> anyhow::Result<String> {
    match file.content_type() {
        Some(ct) => match ct.extension() {
            Some(ext) => Ok(ext.as_str().to_lowercase()),
            None => {
                error!("Failed to extract file extension.");
                bail!("Failed to extract file extension.")
            }
        },
        None => {
            error!("Failed to get content type.");
            bail!("Failed to get content type.")
        }
    }
}
