use serde::{Deserialize, Serialize};

use crate::public::abstract_data::AbstractData;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataBaseTimestamp {
    pub abstract_data: AbstractData,
    pub timestamp: u128,
}

impl DataBaseTimestamp {
    pub fn new(abstract_data: AbstractData, priority_list: &[&str]) -> Self {
        let timestamp = abstract_data.compute_timestamp(priority_list);
        Self {
            abstract_data,
            timestamp,
        }
    }
}
