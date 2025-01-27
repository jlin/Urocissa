use crate::public::{album::Album, database_struct::database::definition::Database, redb::{ALBUM_TABLE, DATA_TABLE}};

use redb::ReadOnlyTable;
use std::error::Error;

use super::Tree;

impl Tree {
    pub fn api_read_tree(&self) -> ReadOnlyTable<&str, Database> {
        self.in_disk
            .begin_read()
            .unwrap()
            .open_table(DATA_TABLE)
            .unwrap()
    }
    pub fn api_read_album(&self) -> ReadOnlyTable<&str, Album> {
        self.in_disk
            .begin_read()
            .unwrap()
            .open_table(ALBUM_TABLE)
            .unwrap()
    }
    pub fn insert_tree_api(&self, data_vec: &Vec<Database>) -> Result<(), Box<dyn Error>> {
        let txn = self.in_disk.begin_write()?;
        for data in data_vec {
            let mut table = txn.open_table(DATA_TABLE)?;
            table.insert(&*data.hash, data)?;
        }
        txn.commit()?;
        Ok(())
    }
}
