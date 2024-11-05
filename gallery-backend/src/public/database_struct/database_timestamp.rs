use serde::{Deserialize, Serialize};

use crate::public::abstract_data::AbstractData;

use super::database::definition::DataBase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataBaseTimestamp {
    pub database: AbstractData,
    pub timestamp: u128,
}

impl DataBaseTimestamp {
    pub fn new(database: AbstractData, priority_list: &Vec<&str>) -> Self {
        let timestamp = database.compute_timestamp(priority_list);
        Self {
            database,
            timestamp,
        }
    }
}
