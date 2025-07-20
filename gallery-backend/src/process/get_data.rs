use crate::{
    operations::open_db::{
        abstract_data_to_database_timestamp_return, clear_abstract_data_metadata,
        hash_to_abstract_data, index_to_hash, open_data_and_album_tables, open_tree_snapshot_table,
    },
    public::structure::database_struct::database_timestamp::DataBaseTimestampReturn,
};
use anyhow::Result;
use anyhow::anyhow;
use rayon::prelude::*;

pub fn get_data_process(
    timestamp: u128,
    start: usize,
    end: usize,
    show_download: bool,
    show_metadata: bool,
) -> Result<Vec<DataBaseTimestampReturn>> {
    // Open tables and snapshot once, exactly as before.
    let (data_table, album_table) = open_data_and_album_tables();
    let tree_snapshot = open_tree_snapshot_table(timestamp)?;
    let end = end.min(tree_snapshot.len());

    if start >= end {
        return Ok(vec![]);
    }

    let database_timestamp_return_list: Result<Vec<DataBaseTimestampReturn>> = (start..end)
        .into_par_iter()
        .map(|index| {
            let hash = index_to_hash(&tree_snapshot, index)
                .map_err(|e| anyhow!("Failed to read hash by index {}: {}", index, e))?;

            let mut abstract_data = hash_to_abstract_data(&data_table, &album_table, hash)
                .map_err(|e| anyhow!("Failed to read abstract data by hash {}: {}", hash, e))?;

            clear_abstract_data_metadata(&mut abstract_data, show_metadata);

            let database_timestamp_return =
                abstract_data_to_database_timestamp_return(abstract_data, timestamp, show_download);
            Ok(database_timestamp_return)
        })
        .collect();

    database_timestamp_return_list
}
