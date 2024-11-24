use crate::public::database_struct::database::definition::DataBase;
use crate::public::tree::start_loop::SHOULD_RESET;
use crate::public::tree::TREE;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[put("/put/generate_random_data?<number>")]
pub async fn generate_random_data(number: usize) {
    let data_vec: Vec<DataBase> = (0..number)
        .into_par_iter()
        .map(|_| DataBase::generate_random_data())
        .collect();
    TREE.insert_tree_api(&data_vec).unwrap();
    SHOULD_RESET.notify_one();
    info!("Insert random data complete")
}
