use crate::public::config::PRIVATE_CONFIG;
use crate::public::redb::DATA_TABLE;
use crate::public::tree::TREE;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::{fs::NamedFile, http::CookieJar};
use rocket_seek_stream::SeekStream;
use std::path::{Path, PathBuf};
#[derive(Responder)]
pub enum CompressedFileResponse<'a> {
    SeekStream(SeekStream<'a>),
    NamedFile(NamedFile),
}

#[get("/object/compressed/<file_path..>")]
pub async fn compressed_file(
    cookies: &CookieJar<'_>,
    file_path: PathBuf,
) -> Result<CompressedFileResponse<'static>, Status> {
    let file_hash = file_path
        .file_stem()
        .and_then(std::ffi::OsStr::to_str)
        .ok_or(Status::BadRequest)?;

    let password_cookie = cookies.get("password");
    let share_id_cookie = cookies.get("share");
    let share_id = share_id_cookie.map_or("", |cookie| cookie.value());

    if !password_cookie.map_or(false, |cookie| cookie.value() == PRIVATE_CONFIG.password) {
        // Handle TREE_DB lookup and early return if any condition fails
        let read_txn = TREE.in_disk.begin_read().unwrap();
        let read_table = read_txn.open_table(DATA_TABLE).unwrap();
        let database = read_table
            .get(&*file_hash.to_string())
            .map_err(|_| Status::InternalServerError)?
            .ok_or(Status::NotFound)?
            .value();
        if !database.album.contains(share_id) {
            println!(
                "Database album {:?} does not contain share ID {:?}",
                database.album, share_id
            );
            return Err(Status::NotFound);
        }
    }

    // Handle file serving outside of the password check and data retrieval block
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
