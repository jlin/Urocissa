use crate::coordinator::Task;
use crate::router::fairing::guard_auth::GuardAuth;
use crate::router::fairing::guard_read_only_mode::GuardReadOnlyMode;
use crate::structure::database_struct::database::definition::Database;
use crate::{TREE, coordinator::COORDINATOR};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[get("/put/generate_random_data?<number>")]
pub async fn generate_random_data(
    _auth: GuardAuth,
    _read_only_mode: GuardReadOnlyMode,
    number: usize,
) {
    let data_vec: Vec<Database> = (0..number)
        .into_par_iter()
        .map(|_| Database::generate_random_data())
        .collect();
    TREE.insert_tree_api(&data_vec).unwrap();
    COORDINATOR.submit(Task::Update());
    info!("Insert random data complete")
}
