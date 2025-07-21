use crate::{
    operations::transitor::{
        hash_to_abstract_data, hash_to_album, hash_to_database, index_to_hash,
    },
    public::{
        db::tree_snapshot::read_tree_snapshot::MyCow,
        structure::{
            abstract_data::AbstractData, album::Album,
            database_struct::database::definition::Database,
        },
    },
};
use anyhow::{Result, anyhow};
use redb::ReadOnlyTable;
pub fn index_to_database(
    tree_snapshot: &MyCow,
    data_table: &ReadOnlyTable<&'static str, Database>,
    index: usize,
) -> Result<Database> {
    let hash = index_to_hash(&tree_snapshot, index)
        .map_err(|e| anyhow!("Failed to read hash by index {}: {}", index, e))?;
    let data = hash_to_database(data_table, hash)
        .map_err(|e| anyhow!("Failed to read database by hash {}: {}", hash, e))?;
    Ok(data)
}

pub fn index_to_abstract_database(
    tree_snapshot: &MyCow,
    data_table: &ReadOnlyTable<&'static str, Database>,
    index: usize,
) -> Result<AbstractData> {
    let hash = index_to_hash(&tree_snapshot, index)
        .map_err(|e| anyhow!("Failed to read hash by index {}: {}", index, e))?;
    let data = hash_to_database(data_table, hash)
        .map_err(|e| anyhow!("Failed to read database by hash {}: {}", hash, e))?;
    Ok(AbstractData::Database(data))
}

pub fn index_to_album(
    tree_snapshot: &MyCow,
    album_table: &ReadOnlyTable<&'static str, crate::public::structure::album::Album>,
    index: usize,
) -> Result<Album> {
    let hash = index_to_hash(&tree_snapshot, index)
        .map_err(|e| anyhow!("Failed to read hash by index {}: {}", index, e))?;
    let album = hash_to_album(album_table, hash)
        .map_err(|e| anyhow!("Failed to read album by hash {}: {}", hash, e))?;
    Ok(album)
}
pub fn index_to_abstract_album(
    tree_snapshot: &MyCow,
    album_table: &ReadOnlyTable<&'static str, crate::public::structure::album::Album>,
    index: usize,
) -> Result<AbstractData> {
    let hash = index_to_hash(&tree_snapshot, index)
        .map_err(|e| anyhow!("Failed to read hash by index {}: {}", index, e))?;
    let album = hash_to_album(album_table, hash)
        .map_err(|e| anyhow!("Failed to read album by hash {}: {}", hash, e))?;
    Ok(AbstractData::Album(album))
}

pub fn index_to_abstract_data(
    tree_snapshot: &MyCow,
    data_table: &ReadOnlyTable<&'static str, Database>,
    album_table: &ReadOnlyTable<&'static str, Album>,
    index: usize,
) -> Result<crate::public::structure::abstract_data::AbstractData> {
    let hash = index_to_hash(&tree_snapshot, index)
        .map_err(|e| anyhow!("Failed to read hash by index {}: {}", index, e))?;
    let abstract_data = hash_to_abstract_data(data_table, album_table, hash)
        .map_err(|e| anyhow!("Failed to read abstract data by hash {}: {}", hash, e))?;
    Ok(abstract_data)
}
