pub mod databaser;

use crate::indexer;
use crate::looper::{LOOPER, Signal};
use crate::structure::database_struct::database::definition::Database;
use crate::tui::{DASHBOARD, FileType};

pub fn indexer(database: Database) -> anyhow::Result<()> {
    let hash = database.hash;
    let newest_path = database.alias.iter().max().unwrap().file.clone();
    DASHBOARD.add_task(
        hash,
        newest_path,
        FileType::try_from(database.ext_type.as_str())?,
    );

    indexer::databaser::databaser(database)?;
    LOOPER.notify(Signal::UpdateTree);
    DASHBOARD.advance_task_state(&hash);

    Ok(())
}
