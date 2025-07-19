use redb::ReadOnlyTable;

use crate::public::{
    constant::redb::DATA_TABLE,
    db::tree::TREE,
    structure::{album::Album, database_struct::database::definition::Database},
};

pub fn open_data_table() -> ReadOnlyTable<&'static str, Database> {
    let read_txn = TREE.in_disk.begin_read().unwrap();
    let data_table = read_txn.open_table(DATA_TABLE).unwrap();
    data_table
}

pub fn open_album_table() -> ReadOnlyTable<&'static str, Album> {
    let read_txn = TREE.in_disk.begin_read().unwrap();
    let album_table = read_txn
        .open_table(crate::public::constant::redb::ALBUM_TABLE)
        .unwrap();
    album_table
}

pub fn open_data_and_album_tables() -> (
    ReadOnlyTable<&'static str, Database>,
    ReadOnlyTable<&'static str, Album>,
) {
    let read_txn = TREE.in_disk.begin_read().unwrap();
    let data_table = read_txn.open_table(DATA_TABLE).unwrap();
    let album_table = read_txn
        .open_table(crate::public::constant::redb::ALBUM_TABLE)
        .unwrap();
    (data_table, album_table)
}
