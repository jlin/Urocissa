use crate::{
    public::{
        constant::redb::DATA_TABLE, db::tree::TREE,
        structure::database_struct::database::definition::Database,
    },
    tasks::batcher::{QueueApi, update_tree::UPDATE_TREE_QUEUE},
};

pub static FLUSH_TREE_QUEUE: QueueApi<Database> = QueueApi::new(flush_tree_task);

fn flush_tree_task(vec: Vec<Database>) {
    let write_txn = TREE.in_disk.begin_write().unwrap();
    {
        let mut write_table = write_txn.open_table(DATA_TABLE).unwrap();
        vec.iter().for_each(|database| {
            write_table.insert(&*database.hash, database).unwrap();
        });
    };
    write_txn.commit().unwrap();
    UPDATE_TREE_QUEUE.update(vec![()]);
}

pub struct FlushTreeTask {
    pub databases: Vec<Database>,
}

impl FlushTreeTask {
    pub fn new(databases: Vec<Database>) -> Self {
        Self { databases }
    }
}
impl mini_coordinator::BatchTask for FlushTreeTask {
    fn batch_run(list: Vec<Self>) -> impl std::future::Future<Output = ()> + Send {
        async move {
            let mut all_databases = Vec::new();
            for task in list {
                all_databases.extend(task.databases);
            }
            flush_tree_task(all_databases);
        }
    }
}
