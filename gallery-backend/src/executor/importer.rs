use indicatif::{ProgressBar, ProgressStyle};
use std::fs;

use crate::public::database_struct::hash_alias::HashAliasSize;

pub fn import(
    deduplicated_file_list: &Vec<HashAliasSize>,
) -> Result<(), Box<dyn std::error::Error>> {
    let progress_bar = ProgressBar::new(deduplicated_file_list.len() as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({eta})")?
            .progress_chars("##-"),
    );
    for hash_alias_size in deduplicated_file_list {
        let source_path = &hash_alias_size.hash_alias.source_path();
        let dest_path = &hash_alias_size.hash_alias.imported_path();
        if let Err(err) = fs::create_dir_all(dest_path.parent().unwrap()) {
            error!("Failed to create directory: {:?}", err);
            return Err(Box::new(err));
        }
        if let Err(err) = fs::copy(&source_path, &dest_path) {
            error!("Failed to copy file: {:?}", err);

            error!("{:?}", source_path);
            return Err(Box::new(err));
        }
        progress_bar.inc(1);
    }
    progress_bar.finish();
    Ok(())
}
