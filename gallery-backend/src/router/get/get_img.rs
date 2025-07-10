use rocket::fs::NamedFile;
use rocket::response::Responder;
use rocket_seek_stream::SeekStream;
use std::path::{Path, PathBuf};

use crate::router::{
    AppResult,
    fairing::{
        guard_hash::{GuardHash, GuardHashOriginal},
        guard_share::GuardShare,
    },
};
#[derive(Responder)]
pub enum CompressedFileResponse<'a> {
    SeekStream(SeekStream<'a>),
    NamedFile(NamedFile),
}

#[get("/object/compressed/<file_path..>")]
pub async fn compressed_file(
    _auth_guard: GuardShare,
    _hash_guard: GuardHash,
    file_path: PathBuf,
) -> AppResult<CompressedFileResponse<'static>> {
    let compressed_file_path = Path::new("./object/compressed").join(&file_path);
    let result = if compressed_file_path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        == Some("mp4")
    {
        SeekStream::from_path(compressed_file_path)
            .map(CompressedFileResponse::SeekStream)
            .map_err(|error| {
                error!("Error opening MP4 file: {:#?}", error);
                anyhow::anyhow!("Error opening MP4 file: {:#?}", error)
            })?
    } else {
        let named_file = NamedFile::open(compressed_file_path)
            .await
            .map_err(|error| {
                error!("Error opening file: {:#?}", error);
                anyhow::anyhow!("Error opening file: {:#?}", error)
            })?;
        CompressedFileResponse::NamedFile(named_file)
    };
    Ok(result)
}

#[get("/object/imported/<file_path..>")]
pub async fn imported_file(
    _auth: GuardShare,
    _hash_guard: GuardHashOriginal,
    file_path: PathBuf,
) -> AppResult<CompressedFileResponse<'static>> {
    let imported_file_path = Path::new("./object/imported").join(&file_path);

    NamedFile::open(imported_file_path)
        .await
        .map(CompressedFileResponse::NamedFile)
        .map_err(|error| {
            error!("Error opening imported file: {:#?}", error);
            anyhow::anyhow!("Error opening imported file: {:#?}", error).into()
        })
}
