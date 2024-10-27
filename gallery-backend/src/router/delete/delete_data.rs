use crate::public::redb::DATA_TABLE;
use crate::public::tree::start_loop::SHOULD_RESET;
use crate::public::tree::TREE;
use crate::public::tree_snapshot::TREE_SNAPSHOT;
use redb::ReadableTable;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::{json::Json, Deserialize};
use std::sync::atomic::Ordering;
#[derive(Debug, Deserialize)]
pub struct DeleteList {
    #[serde(rename = "deleteList")]
    delete_list: Vec<usize>, // Keep this field private as it does not need to be accessed outside this struct
    timestamp: String,
}
#[delete("/delete/delete-data", format = "json", data = "<json_data>")]
pub async fn delete_data(json_data: Json<DeleteList>) -> Result<(), Custom<String>> {
    println!("get data {:?}", json_data);

    tokio::task::spawn_blocking(move || {
        let timestamp = &json_data.timestamp;
        let tree_snapshot = match TREE_SNAPSHOT.read_tree_snapshot(timestamp) {
            Ok(snapshot) => snapshot,
            Err(_) => {
                return Err(Custom(
                    Status::InternalServerError,
                    "Failed to read tree snapshot".to_string(),
                ));
            }
        };

        let txn = match TREE.in_disk.begin_write() {
            Ok(transaction) => transaction,
            Err(_) => {
                return Err(Custom(
                    Status::InternalServerError,
                    "Failed to begin write transaction".to_string(),
                ));
            }
        };

        {
            let mut table = match txn.open_table(DATA_TABLE) {
                Ok(table) => table,
                Err(_) => {
                    return Err(Custom(
                        Status::InternalServerError,
                        "Failed to open data table".to_string(),
                    ));
                }
            };

            for index in &json_data.delete_list {
                let hash = tree_snapshot.get_hash(*index);

                let data = match table.get(hash.as_str()) {
                    Ok(Some(record)) => record.value(),
                    Ok(None) => {
                        return Err(Custom(
                            Status::InternalServerError,
                            format!("Data not found for hash: {}", hash),
                        ));
                    }
                    Err(_) => {
                        return Err(Custom(
                            Status::InternalServerError,
                            "Failed to retrieve data from table".to_string(),
                        ));
                    }
                };

                let compressed_path = data.compressed_path();
                let imported_path = data.imported_path();

                if std::fs::remove_file(&compressed_path).is_err() {
                    eprintln!("Failed to delete file at {:?}", compressed_path);
                    return Err(Custom(
                        Status::InternalServerError,
                        format!("Failed to delete file at {:?}", compressed_path),
                    ));
                }
                if std::fs::remove_file(&imported_path).is_err() {
                    eprintln!("Failed to delete file at {:?}", imported_path);
                    return Err(Custom(
                        Status::InternalServerError,
                        format!("Failed to delete file at {:?}", imported_path),
                    ));
                }

                match table.remove(hash.as_str()) {
                    Ok(_) => (),
                    Err(err) => {
                        println!("{:?}", err);
                        return Err(Custom(
                            Status::InternalServerError,
                            "Failed to remove data from table".to_string(),
                        ));
                    }
                };
            }
        }

        if txn.commit().is_err() {
            return Err(Custom(
                Status::InternalServerError,
                "Failed to commit transaction".to_string(),
            ));
        }

        SHOULD_RESET.store(true, Ordering::SeqCst);
        Ok(())
    })
    .await
    .unwrap()
}
