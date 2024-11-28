use std::sync::atomic::Ordering;

use super::TreeSnapshot;
use crate::{
    public::{reduced_data::ReducedData, tree::start_loop::VERSION_COUNT},
    router::get::get_data::Prefetch,
};
use arrayvec::ArrayString;
use dashmap::mapref::one::Ref;

use redb::{ReadOnlyTable, ReadableTableMetadata, TableDefinition};
use rocket::http::Status;

impl TreeSnapshot {
    pub fn read_query_snapshot(
        &'static self,
        query_hash: u64,
    ) -> Result<Option<Option<Prefetch>>, Status> {
        if let Some(data) = self.expression_timestamp_in_memory.get(&query_hash) {
            return Ok(Some(data.value().clone()));
        }

        let read_txn = self.in_disk.begin_read().map_err(|err| {
            error!("{:?}", err);
            Status::InternalServerError
        })?;

        let count_version = &VERSION_COUNT.load(Ordering::Relaxed).to_string();

        print!("try to read count_version {}", count_version);

        let table_definition: TableDefinition<u64, Option<Prefetch>> =
            TableDefinition::new(&count_version);

        let table = read_txn.open_table(table_definition).map_err(|err| {
            error!("{:?}", err);
            Status::InternalServerError
        })?;

        let timestamp = table.get(query_hash).map_err(|err| {
            error!("{:?}", err);
            Status::InternalServerError
        })?;

        Ok(timestamp.map(|inner_value| inner_value.value()))
    }
}
