use redb::ReadableTable;
use rocket::get;
use rocket::response::stream::ByteStream;
use serde::Serialize;

use crate::{
    public::{database_struct::database::definition::DataBase, tree::TREE},
    router::fairing::AuthGuard,
};

#[derive(Debug, Serialize)]
pub struct ExportEntry {
    key: String,
    value: DataBase,
}

#[get("/get/get-export")]
pub async fn get_export(_auth: AuthGuard) -> ByteStream![Vec<u8>] {
    ByteStream! {
        let table = TREE.api_read_tree();

        let iter = match table.iter() {
            Ok(it) => it,
            Err(_) => {
                yield b"{\"error\":\"failed to iterate\"}".to_vec();
                return;
            }
        };

        // Start the JSON array, with a newline
        yield b"[\n".to_vec();
        let mut first = true;

        for entry_res in iter {
            let (key, value) = match entry_res {
                Ok(kv) => kv,
                Err(_) => {
                    // Skip or handle the error
                    continue;
                }
            };

            // If not the first element, insert a comma + newline
            if !first {
                yield b",\n".to_vec();
            }
            first = false;

            // Build the ExportEntry
            let export = ExportEntry {
                key: key.value().to_string(),
                value: value.value().clone(),
            };

            // Pretty-print each individual object to JSON
            let json_obj = match serde_json::to_string_pretty(&export) {
                Ok(s) => s,
                Err(_) => {
                    // Skip or handle the error
                    continue;
                }
            };

            // Stream it out
            yield json_obj.into_bytes();
        }

        // End the JSON array with a final newline
        yield b"\n]\n".to_vec();
    }
}
