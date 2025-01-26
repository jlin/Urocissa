use redb::TableDefinition;

use crate::public::database_struct::database::definition::DataBase;

use super::album::Album;

pub const DATA_TABLE: TableDefinition<&str, DataBase> = TableDefinition::new("data");

// FIXME: Typo in table name "albumm". Changing this now will cause database corruption.
pub const ALBUM_TABLE: TableDefinition<&str, Album> = TableDefinition::new("albumm");

pub const SCHEMA_TABLE: TableDefinition<&str, u8> = TableDefinition::new("schema");
