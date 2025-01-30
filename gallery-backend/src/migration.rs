use crate::public::redb::SCHEMA_TABLE;
use log::{info, warn};
use redb::ReadableTable;
use redb::WriteTransaction;

pub fn check_database_schema_version() {
    let version = read_version();

    info!("Database schema vesrion: {}", version);

    if version == 0 {
        warn!("Perform database migration");
        migration();
        info!("Database migration completed.")
    }
}

pub fn read_version() -> u8 {
    let database = redb::Database::create("./db/index.redb").unwrap();
    let version: u8;
    let txn = database.begin_write().unwrap();
    {
        let version_table = txn.open_table(SCHEMA_TABLE).unwrap();

        // Read "version" or default to 0
        version = version_table
            .get("version")
            .unwrap()
            .map(|guard| guard.value())
            .unwrap_or(0);
    }
    txn.commit().unwrap();
    version
}

pub fn migration() {
    let database = redb::Database::create("./db/index.redb")
        .expect("Migration failed: Unable to create database");

    let txn = database
        .begin_write()
        .expect("Migration failed: Unable to begin write transaction");

    migration_database(&txn);
    migration_album(&txn);
    migration_version(&txn);

    txn.commit()
        .expect("Migration failed: Unable to commit transaction");
}

pub fn migration_album(txn: &WriteTransaction) {
    let mut new_table = txn
        .open_table(crate::public::redb::ALBUM_TABLE)
        .expect("Migration failed: Unable to open new table");
    let old_table = txn
        .open_table(urocissa::ALBUM_TABLE)
        .expect("Migration failed: Unable to open old table");

    old_table
        .iter()
        .expect("Migration failed: Unable to iterate over old table")
        .for_each(|result| {
            let (_, value_guard) =
                result.expect("Migration failed: Unable to retrieve value from old table");
            let old_album = value_guard.value();
            let converted = crate::public::album::Album {
                id: old_album.id,
                title: old_album.title,
                created_time: old_album.created_time,
                start_time: old_album.start_time,
                end_time: old_album.end_time,
                last_modified_time: old_album.last_modified_time,
                cover: old_album.cover,
                thumbhash: None,
                user_defined_metadata: old_album.user_defined_metadata,
                share_list: old_album
                    .share_list
                    .into_iter()
                    .map(|share| crate::public::album::Share {
                        url: share.url,
                        description: share.description,
                        password: share.password,
                        show_metadata: share.show_metadata,
                        show_download: share.show_download,
                        show_upload: share.show_upload,
                        exp: share.exp,
                    })
                    .collect(),
                tag: old_album.tag,
                width: old_album.width,
                height: old_album.height,
                item_count: old_album.item_count,
                item_size: old_album.item_size,
                pending: old_album.pending,
            };
            new_table
                .insert(&*old_album.id, converted)
                .expect("Migration failed: Unable to insert data into new table");
        });
    txn.delete_table(old_table)
        .expect("Migration failed: Unable to delete old table");
}

pub fn migration_database(txn: &WriteTransaction) {
    let mut new_table = txn
        .open_table(crate::public::redb::DATA_TABLE)
        .expect("Migration failed: Unable to open new table");
    let old_table = txn
        .open_table(urocissa::DATA_TABLE)
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

pub fn migration_version(txn: &WriteTransaction) {
    let mut version_table = txn.open_table(SCHEMA_TABLE).unwrap();
    version_table
        .insert("version", 1)
        .expect("Migration failed: Unable to insert number to version table");
}
