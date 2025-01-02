use crate::executor::databaser::generate_compressed_image::regenerate_compressed_image;
use crate::public::constant::PROCESS_BATCH_NUMBER;
use crate::public::tree::TREE;
use crate::public::tree_snapshot::TREE_SNAPSHOT;
use crate::router::fairing::{AuthGuard, ReadOnlyModeGuard};
use arrayvec::ArrayString;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rocket::form::Form;
use rocket::form::{self, DataField, FromFormField, ValueField};
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegenerateData {
    index_array: Vec<usize>,
    timestamp: u128,
}

#[put("/put/regenerate-thumbnail", format = "json", data = "<json_data>")]
pub async fn regenerate_thumbnail(
    _auth: AuthGuard,
    _read_only_mode: ReadOnlyModeGuard,
    json_data: Json<RegenerateData>,
) -> Status {
    tokio::task::spawn_blocking(move || {
        let table = TREE.read_tree_api();

        let reduced_data_vec = TREE_SNAPSHOT
            .read_tree_snapshot(&json_data.timestamp)
            .unwrap();

        let hash_vec: Vec<ArrayString<64>> = json_data
            .index_array
            .par_iter()
            .map(|index| reduced_data_vec.get_hash(*index))
            .collect();
        let total_batches = (hash_vec.len() + PROCESS_BATCH_NUMBER - 1) / PROCESS_BATCH_NUMBER;

        for (i, batch) in hash_vec.chunks(PROCESS_BATCH_NUMBER).enumerate() {
            info!("Processing batch {}/{}", i + 1, total_batches);

            batch.into_par_iter().for_each(|string| {
                let mut database = table.get(&**string).unwrap().unwrap().value();

                regenerate_compressed_image(&mut database).unwrap();
            });
        }
    });

    // Return 200 OK immediately
    Status::Ok
}
pub enum FrameData<'r> {
    Hash(ArrayString<64>),
    File(TempFile<'r>),
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for FrameData<'r> {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        let valid_hash = String::from_value(field)?;
        match ArrayString::<64>::from(&valid_hash) {
            Ok(hash) => Ok(FrameData::Hash(hash)),
            Err(_) => Err(form::Error::validation("Invalid hash length or format").into()),
        }
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        match TempFile::from_data(field).await {
            Ok(temp_file) => Ok(FrameData::File(temp_file)),
            Err(err) => Err(err),
        }
    }
}

#[put("/put/regenerate-thumbnail-with-frame", data = "<data>")]
pub async fn regenerate_thumbnail_with_frame(
    _auth: AuthGuard,
    _read_only_mode: ReadOnlyModeGuard,
    data: Form<Vec<FrameData<'_>>>,
) -> Result<Status, Status> {
    let mut hash: Option<ArrayString<64>> = None;

    for frame_data in data.into_inner() {
        match frame_data {
            FrameData::Hash(received_hash) => {
                hash = Some(received_hash);
            }
            FrameData::File(mut file) => {
                let hash = hash.unwrap();
                let file_path = format!("./object/compressed/{}/{}.jpg", &hash[0..2], hash);

                if let Err(e) = file.move_copy_to(file_path).await {
                    eprintln!("Failed to save file: {:?}", e);
                    return Err(Status::InternalServerError);
                }
            }
        }
    }

    info!("Regenerating thumbnail successfully");

    Ok(Status::Ok)
}
