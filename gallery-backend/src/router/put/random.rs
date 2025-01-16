use crate::public::tree::TREE;
use crate::router::fairing::ReadOnlyModeGuard;
use crate::{public::database_struct::database::definition::DataBase, router::fairing::AuthGuard};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[get("/put/generate_random_data?<number>")]
pub async fn generate_random_data(
    _auth: AuthGuard,
    _read_only_mode: ReadOnlyModeGuard,
    number: usize,
) {
    let data_vec: Vec<DataBase> = (0..number)
        .into_par_iter()
        .map(|_| DataBase::generate_random_data())
        .collect();
    TREE.insert_tree_api(&data_vec).unwrap();
    TREE.tree_update();
    info!("Insert random data complete")
}
