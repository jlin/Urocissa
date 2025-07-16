use tokio_rayon::spawn;

use crate::{
    coordinator::actor::Task, indexer::indexer,
    structure::database_struct::database::definition::Database,
};

pub struct IndexTask {
    pub database: Database,
}

impl IndexTask {
    pub fn new(database: Database) -> Self {
        Self { database }
    }
}

impl Task for IndexTask {
    type Output = anyhow::Result<()>;

    fn run(self) -> impl std::future::Future<Output = Self::Output> + Send {
        async move {
            let result = spawn(move || indexer(self.database))
                .await
                .expect("blocking task panicked");
            Ok(result)
        }
    }
}
