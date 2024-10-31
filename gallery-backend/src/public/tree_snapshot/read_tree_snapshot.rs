use super::TreeSnapshot;
use crate::public::tree_snap_shot_in_memory::ReducedData;
use arrayvec::ArrayString;
use dashmap::mapref::one::Ref;

use redb::{ReadOnlyTable, ReadableTableMetadata, TableDefinition};
use rocket::http::Status;

impl TreeSnapshot {
    pub fn read_tree_snapshot(&'static self, timestamp: &str) -> Result<MyCow, Status> {
        if let Some(data) = self.in_memory.get(timestamp) {
            return Ok(MyCow::DashMap(data));
        }

        let read_txn = self.in_disk.begin_read().map_err(|err| {
            error!("{:?}", err);
            Status::InternalServerError
        })?;

        let table_definition: TableDefinition<u64, ReducedData> = TableDefinition::new(&timestamp);

        let table = read_txn.open_table(table_definition).map_err(|err| {
            error!("{:?}", err);
            Status::InternalServerError
        })?;

        Ok(MyCow::Redb(table))
    }
}

#[derive(Debug)]
pub enum MyCow {
    DashMap(Ref<'static, String, Vec<ReducedData>>),
    Redb(ReadOnlyTable<u64, ReducedData>),
}

impl MyCow {
    pub fn len(&self) -> usize {
        match self {
            MyCow::DashMap(data) => data.value().len(),
            MyCow::Redb(table) => table.len().unwrap() as usize,
        }
    }

    pub fn get_width_height(&self, index: usize) -> (u32, u32) {
        match self {
            MyCow::DashMap(data) => {
                let data = &data.value()[index];
                (data.width, data.height)
            }
            MyCow::Redb(table) => {
                let data = &table.get(index as u64).unwrap().unwrap().value();
                (data.width, data.height)
            }
        }
    }

    pub fn get_hash(&self, index: usize) -> ArrayString<64> {
        match self {
            MyCow::DashMap(data) => {
                let data = &data.value()[index];
                data.hash
            }
            MyCow::Redb(table) => {
                let data = table.get(index as u64).unwrap().unwrap().value();
                data.hash
            }
        }
    }
}
