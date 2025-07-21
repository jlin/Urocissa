use crate::public::{
    constant::redb::DATA_TABLE,
    db::{
        tree::TREE,
        tree_snapshot::{TREE_SNAPSHOT, read_tree_snapshot::MyCow},
    },
    structure::{
        abstract_data::AbstractData,
        album::Album,
        database_struct::{
            database::definition::Database, database_timestamp::DataBaseTimestampReturn,
        },
    },
};
use anyhow::Context;
use anyhow::Result;
use arrayvec::ArrayString;
use redb::ReadOnlyTable;

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

pub fn open_tree_snapshot_table(timestamp: u128) -> Result<MyCow> {
    TREE_SNAPSHOT
        .read_tree_snapshot(&timestamp)
        .context(format!(
            "Failed to read tree snapshot for timestamp {}",
            timestamp
        ))
}

pub fn index_to_hash(tree_snapshot: &MyCow, index: usize) -> Result<ArrayString<64>> {
    if index >= tree_snapshot.len() {
        return Err(anyhow::anyhow!("Index out of bounds: {}", index));
    }
    let hash = tree_snapshot.get_hash(index)?;
    Ok(hash)
}

pub fn hash_to_database(
    data_table: &ReadOnlyTable<&'static str, Database>,
    hash: ArrayString<64>,
) -> Result<Database> {
    if let Some(database) = data_table.get(&*hash)? {
        let database = database.value();
        Ok(database)
    } else {
        Err(anyhow::anyhow!("No data found for hash: {}", hash))
    }
}

pub fn hash_to_album(
    album_table: &ReadOnlyTable<&'static str, Album>,
    hash: ArrayString<64>,
) -> Result<Album> {
    if let Some(album) = album_table.get(&*hash)? {
        let album = album.value();
        Ok(album)
    } else {
        Err(anyhow::anyhow!("No album found for hash: {}", hash))
    }
}

pub fn hash_to_abstract_data(
    data_table: &ReadOnlyTable<&'static str, Database>,
    album_table: &ReadOnlyTable<&'static str, Album>,
    hash: ArrayString<64>,
) -> Result<AbstractData> {
    if let Some(database) = data_table.get(&*hash)? {
        let database = database.value();

        Ok(AbstractData::Database(database))
    } else if let Some(album) = album_table.get(&*hash)? {
        let album = album.value();

        Ok(AbstractData::Album(album))
    } else {
        Err(anyhow::anyhow!("No data found for hash: {}", hash))
    }
}

pub fn clear_abstract_data_metadata(abstract_data: &mut AbstractData, show_metadata: bool) {
    match abstract_data {
        AbstractData::Database(database) => {
            if !show_metadata {
                database.tag.clear();
                database.album.clear();
                database.alias.clear();
            }
        }
        AbstractData::Album(album) => {
            if !show_metadata {
                album.tag.clear();
            }
        }
    }
}

pub fn abstract_data_to_database_timestamp_return(
    abstract_data: AbstractData,
    timestamp: u128,
    show_download: bool,
) -> DataBaseTimestampReturn {
    DataBaseTimestampReturn::new(
        abstract_data,
        &crate::public::constant::DEFAULT_PRIORITY_LIST,
        timestamp,
        show_download,
    )
}
