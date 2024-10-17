use crate::public::database_struct::file_modify::FileModifySize;
use crate::public::database_struct::hash_alias::{Alias, AliasSize};
use crate::public::error_data::{handle_error, ErrorData};
use crate::public::tree::TREE;
use arrayvec::ArrayString;
use blake3::Hasher;
use dashmap::{DashMap, DashSet};
use rayon::prelude::*;
use std::panic::Location;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
};

pub fn validator(
    vec_of_file_modify: DashSet<FileModifySize>,
) -> DashMap<ArrayString<64>, AliasSize> {
    let duplicated_files_number = AtomicUsize::new(0);
    let dashmap_of_hash_alias: DashMap<ArrayString<64>, AliasSize> =
        DashMap::with_capacity(vec_of_file_modify.len());
    let scaned_number = AtomicUsize::new(0);
    vec_of_file_modify
        .into_par_iter()
        .for_each(|file_modify_size| {
            let file_modify = file_modify_size.file_modify;
            match blake3_hasher(&PathBuf::from(&file_modify.file)) {
                Ok(hash) => {
                    let read_table = TREE.read_tree_api();
                    match read_table.get(&*hash).unwrap() {
                        Some(guard) => {
                            let mut database = guard.value();
                            database.alias.push(file_modify);
                            TREE.insert_tree_api(&vec![database]).unwrap();
                            scaned_number.fetch_add(1, Ordering::SeqCst);
                        }
                        None => {
                            //if this is a new file
                            //if the new file is duplicated in this batch
                            if let Some(mut duplicated_alias) = dashmap_of_hash_alias.get_mut(&hash)
                            {
                                duplicated_alias.alias.alias.push(file_modify);
                                duplicated_files_number.fetch_add(1, Ordering::SeqCst);
                            } else {
                                dashmap_of_hash_alias.insert(
                                    hash,
                                    AliasSize::new(
                                        Alias::new(vec![file_modify]),
                                        file_modify_size.size,
                                    ),
                                );
                            }
                        }
                    }
                }
                Err(err) => {
                    handle_error(err);
                }
            }
        });
    dashmap_of_hash_alias
}

fn blake3_hasher(file_path: &Path) -> Result<ArrayString<64>, ErrorData> {
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(err) => {
            return Err(ErrorData::new(
                err.to_string(),
                format!("Failed to read file"),
                None,
                Some(file_path.to_path_buf()),
                Location::caller(),
            ));
        }
    };
    let mut hasher = Hasher::new();
    let mut buffer = [0; 512 * 1024];
    loop {
        match file.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }
                hasher.update(&buffer[..bytes_read]);
            }
            Err(err) => {
                return Err(ErrorData::new(
                    err.to_string(),
                    format!("fail to read file"),
                    None,
                    Some(file_path.to_path_buf()),
                    Location::caller(),
                ));
            }
        }
    }
    let hash = hasher.finalize();
    Ok(hash.to_hex())
}
