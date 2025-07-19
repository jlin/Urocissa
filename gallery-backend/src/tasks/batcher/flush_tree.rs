use crate::{
    public::{
        constant::redb::DATA_TABLE, db::tree::TREE,
        structure::database_struct::database::definition::Database,
    },
    tasks::{COORDINATOR, batcher::update_tree::UpdateTreeTask},
};

pub struct FlushTreeTask {
    pub insert_list: Vec<Database>,
    pub remove_list: Vec<Database>,
}

impl FlushTreeTask {
    pub fn insert(databases: Vec<Database>) -> Self {
        Self {
            insert_list: databases,
            remove_list: Vec::new(),
        }
    }
    pub fn remove(databases: Vec<Database>) -> Self {
        Self {
            insert_list: Vec::new(),
            remove_list: databases,
        }
    }
}
impl mini_executor::BatchTask for FlushTreeTask {
    fn batch_run(list: Vec<Self>) -> impl std::future::Future<Output = ()> + Send {
        async move {
            let mut all_insert_databases = Vec::new();
            let mut all_remove_databases = Vec::new();
            for task in list {
                all_insert_databases.extend(task.insert_list);
                all_remove_databases.extend(task.remove_list);
            }
            flush_tree_task(all_insert_databases, all_remove_databases);
        }
    }
}

fn flush_tree_task(insert_list: Vec<Database>, remove_list: Vec<Database>) {
    let write_txn = TREE.in_disk.begin_write().unwrap();
    {
        let mut write_table = write_txn.open_table(DATA_TABLE).unwrap();
        insert_list.iter().for_each(|database| {
            write_table.insert(&*database.hash, database).unwrap();
        });
        remove_list.iter().for_each(|database| {
            write_table.remove(&*database.hash).unwrap();
        });
    };
    write_txn.commit().unwrap();
    COORDINATOR.execute_batch_detached(UpdateTreeTask);
}
