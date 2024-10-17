use serde::{Deserialize, Serialize};

use super::database::definition::DataBase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataBaseTimestamp {
    pub database: DataBase,
    pub timestamp: u128,
}

impl DataBaseTimestamp {
    pub fn new(database: DataBase, priority_list: &Vec<&str>) -> Self {
        let timestamp = database.compute_timestamp(priority_list);
        Self {
            database,
            timestamp,
        }
    }
}
