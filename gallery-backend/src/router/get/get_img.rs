use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::response::Responder;
use rocket_seek_stream::SeekStream;
use std::path::{Path, PathBuf};
#[derive(Responder)]
pub enum CompressedFileResponse<'a> {
    SeekStream(SeekStream<'a>),
    NamedFile(NamedFile),
}

#[get("/object/compressed/<file_path..>")]
pub async fn compressed_file(
    file_path: PathBuf,
) -> Result<CompressedFileResponse<'static>, Status> {
    let compressed_file_path = Path::new("./object/compressed").join(&file_path);
    if compressed_file_path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        == Some("mp4")
    {
        SeekStream::from_path(compressed_file_path)
            .map(CompressedFileResponse::SeekStream)
            .map_err(|error| {
                println!("Error opening MP4 file: {:?}", error);
                Status::NotFound
            })
    } else {
        NamedFile::open(compressed_file_path)
            .await
            .map(CompressedFileResponse::NamedFile)
            .map_err(|error| {
                println!("Error opening file: {:?}", error);
                Status::NotFound
            })
    }
}
