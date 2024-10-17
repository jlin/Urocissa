use redb::TableDefinition;

use crate::public::database_struct::database::definition::DataBase;

pub const DATA_TABLE: TableDefinition<&str, DataBase> = TableDefinition::new("data");
