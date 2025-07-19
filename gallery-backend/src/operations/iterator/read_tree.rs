use crate::public::{
    constant::redb::{ALBUM_TABLE, DATA_TABLE},
    db::tree::TREE,
    structure::{
        abstract_data::AbstractData, album::Album, database_struct::database::definition::Database,
    },
};
use anyhow::Result;
use anyhow::anyhow;
use arrayvec::ArrayString;

/// Read database entries by their hashes
pub fn read_databases_by_hashes(
    hash_list: Vec<ArrayString<64>>,
) -> impl Iterator<Item = Result<Database>> {
    let read_txn = TREE.in_disk.begin_read().unwrap();
    let table = read_txn.open_table(DATA_TABLE).unwrap();

    hash_list
        .into_iter()
        .map(move |hash| match table.get(&*hash) {
            Ok(Some(value)) => Ok(value.value()),
            Ok(None) => Err(anyhow!("Database not found for hash: {}", hash)),
            Err(e) => Err(anyhow!("Database read error for hash {}: {}", hash, e)),
        })
}

/// Read album entries by their hashes
pub fn read_albums_by_hashes(
    hash_list: Vec<ArrayString<64>>,
) -> impl Iterator<Item = Result<Album>> {
    let read_txn = TREE.in_disk.begin_read().unwrap();
    let album_table = read_txn.open_table(ALBUM_TABLE).unwrap();

    hash_list
        .into_iter()
        .map(move |hash| match album_table.get(&*hash) {
            Ok(Some(value)) => Ok(value.value()),
            Ok(None) => Err(anyhow!("Album not found for hash: {}", hash)),
            Err(e) => Err(anyhow!("Album read error for hash {}: {}", hash, e)),
        })
}

/// Read abstract data by hashes, trying database table first, then album table
pub fn read_abstract_data_by_hashes(
    hash_list: Vec<ArrayString<64>>,
) -> impl Iterator<Item = Result<AbstractData>> {
    let read_txn = TREE.in_disk.begin_read().unwrap();
    let data_table = read_txn.open_table(DATA_TABLE).unwrap();
    let album_table = read_txn.open_table(ALBUM_TABLE).unwrap();

    hash_list.into_iter().map(move |hash| {
        let key: &str = &*hash;
        // Inner closure to use `?` and early return.

        // 1) Try the data table
        if let Some(db_val) = data_table
            .get(key)
            .map_err(|e| anyhow!("Database read error for hash {}: {}", key, e))?
        {
            return Ok(AbstractData::Database(db_val.value()));
        }
        // 2) Fallback to the album table
        if let Some(al_val) = album_table
            .get(key)
            .map_err(|e| anyhow!("Album read error for hash {}: {}", key, e))?
        {
            return Ok(AbstractData::Album(al_val.value()));
        }
        // 3) Not found in either
        Err(anyhow!("Data not found in either table for hash: {}", key))
    })
}
