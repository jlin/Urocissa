use crate::{
    public::constant::redb::DATA_TABLE, public::db::tree::TREE,
    public::structure::database_struct::database::definition::Database, tasks::batcher::QueueApi,
};

pub static FLUSH_TREE_QUEUE: QueueApi<Database> = QueueApi::new(flush_tree_task);

pub fn flush_tree_task(vec: Vec<Database>) {
    let write_txn = TREE.in_disk.begin_write().unwrap();

    {
        let mut write_table: redb::Table<'_, &str, Database> =
            write_txn.open_table(DATA_TABLE).unwrap();
        vec.iter().for_each(|database| {
            write_table.insert(&*database.hash, database).unwrap();
        });
    };
    write_txn.commit().unwrap();
}
