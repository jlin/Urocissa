use redb::TableDefinition;

use crate::public::database_struct::database::definition::DataBase;

use super::album::Album;

pub const DATA_TABLE: TableDefinition<&str, DataBase> = TableDefinition::new("data");

pub const ALBUM_TABLE: TableDefinition<&str, Album> = TableDefinition::new("albumm");
