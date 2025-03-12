use crate::public::{database_struct::database::definition::Database, tree::TREE};
use crate::router::fairing::auth_guard::GuardAuth;
use crate::router::fairing::read_only_mod_guard::GuardReadOnlyMode;
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
    TREE.tree_update();
    info!("Insert random data complete")
}
