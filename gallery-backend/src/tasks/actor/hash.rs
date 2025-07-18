use crate::{operations::hash::blake3_hasher, public::error_data::handle_error};
use anyhow::Result;
use arrayvec::ArrayString;
use mini_executor::Task;
use std::fs::File;
use tokio_rayon::spawn;

pub struct HashTask {
    pub file: File,
}

impl HashTask {
    pub fn new(file: File) -> Self {
        Self { file }
    }
}

impl Task for HashTask {
    type Output = Result<ArrayString<64>>;

    fn run(self) -> impl std::future::Future<Output = Self::Output> + Send {
        async move {
            spawn(move || hash_task(self.file))
                .await
                .map_err(|err| handle_error(err.context("Failed to run hash task")))
        }
    }
}
fn hash_task(file: File) -> Result<ArrayString<64>> {
    blake3_hasher(file)
}
