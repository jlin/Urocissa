use crate::public::{database_struct::database::definition::DataBase, redb::DATA_TABLE};

use redb::ReadOnlyTable;
use std::error::Error;

use super::Tree;

impl Tree {
    pub fn read_tree_api(&self) -> ReadOnlyTable<&str, DataBase> {
        self.in_disk
            .begin_read()
            .unwrap()
            .open_table(DATA_TABLE)
            .unwrap()
    }
    pub fn insert_tree_api(&self, data_vec: &Vec<DataBase>) -> Result<(), Box<dyn Error>> {
        let txn = self.in_disk.begin_write()?;
        for data in data_vec {
            let mut table = txn.open_table(DATA_TABLE)?;
            table.insert(&*data.hash, data)?;
        }
        txn.commit()?;
        Ok(())
    }
}
