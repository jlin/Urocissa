use redb::TableDefinition;

use crate::public::database_struct::database::definition::Database;

use super::album::Album;

pub const DATA_TABLE: TableDefinition<&str, Database> = TableDefinition::new("database");

pub const ALBUM_TABLE: TableDefinition<&str, Album> = TableDefinition::new("album");

pub const SCHEMA_TABLE: TableDefinition<&str, u8> = TableDefinition::new("schema");
