use crate::{
    public::constant::redb::{ALBUM_TABLE, DATA_TABLE},
    public::structure::{album::Album, database_struct::database::definition::Database},
};

use redb::ReadOnlyTable;

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
}
