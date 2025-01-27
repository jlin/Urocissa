use log::{info, warn};
use redb::ReadableTable;

use crate::public::redb::{DATA_TABLE, SCHEMA_TABLE};

pub fn check_database_schema_version() {
    let database = redb::Database::create("./db/index.redb").unwrap();

    let txn = database.begin_write().unwrap();
    let version_table = txn.open_table(SCHEMA_TABLE).unwrap();

    // Read "version" or default to 0
    let version = version_table
        .get("version")
        .unwrap()
        .map(|guard| guard.value())
        .unwrap_or(0);

    // Explicitly drop table so it doesn't borrow 'txn' when we commit
    drop(version_table);

    txn.commit().unwrap();

    drop(database);

    info!("Database schema vesrion: {}", version);
    if version == 0 {
        warn!("Perform database migration");
        migration();
    }
    let database = redb::Database::create("./db/index.redb").unwrap();
    let txn = database.begin_write().unwrap();
    {
        let mut version_table = txn.open_table(SCHEMA_TABLE).unwrap();
        version_table
            .insert("version", 1)
            .expect("Migration failed: Unable to insert number to version table");
    }
    txn.commit().unwrap();
}

pub fn migration() {
    let database = redb::Database::create("./db/index.redb")
        .expect("Migration failed: Unable to create database");
    let txn = database
        .begin_write()
        .expect("Migration failed: Unable to begin write transaction");
    {
        let mut new_table = txn
            .open_table(DATA_TABLE)
            .expect("Migration failed: Unable to open new table");
        let old_table = txn
            .open_table(Urocissa::DATA_TABLE)
            .expect("Migration failed: Unable to open old table");

        old_table
            .iter()
            .expect("Migration failed: Unable to iterate over old table")
            .for_each(|result| {
                let (_, value_guard) =
                    result.expect("Migration failed: Unable to retrieve value from old table");
                let old_data = value_guard.value();
                let converted = crate::public::database_struct::database::definition::Database {
                    hash: old_data.hash,
                    size: old_data.size,
                    width: old_data.width,
                    height: old_data.height,
                    thumbhash: old_data.thumbhash,
                    phash: old_data.phash,
                    ext: old_data.ext,
                    exif_vec: old_data.exif_vec,
                    tag: old_data.tag,
                    album: old_data.album,
                    alias: old_data
                        .alias
                        .into_iter()
                        .map(
                            |old_alias| crate::public::database_struct::file_modify::FileModify {
                                file: old_alias.file,
                                modified: old_alias.modified,
                                scan_time: old_alias.scan_time,
                            },
                        )
                        .collect(),
                    ext_type: old_data.ext_type,
                    pending: old_data.pending,
                };
                new_table
                    .insert(&*old_data.hash, converted)
                    .expect("Migration failed: Unable to insert data into new table");
            });
        txn.delete_table(old_table)
            .expect("Migration failed: Unable to delete old table");
    }
    txn.commit()
        .expect("Migration failed: Unable to commit transaction");
    info!("Database migration completed successfully.")
}
