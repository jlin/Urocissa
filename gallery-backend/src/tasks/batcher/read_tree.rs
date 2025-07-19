use arrayvec::ArrayString;
use mini_executor::BatchTask;

use crate::{
    public::{
        constant::redb::DATA_TABLE, db::tree::TREE,
        structure::database_struct::database::definition::Database,
    },
    tasks::{COORDINATOR, batcher::update_tree::UpdateTreeTask},
};

pub struct ReadTreeTask {
    pub hash_list: Vec<ArrayString<64>>,
}

impl ReadTreeTask {
    pub fn new(hash_list: Vec<ArrayString<64>>) -> Self {
        Self { hash_list }
    }
}
impl BatchTask for ReadTreeTask {
    fn batch_run(list: Vec<Self>) -> impl Future<Output = ()> + Send {
        async move { todo!() }
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
