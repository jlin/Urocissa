use std::path::PathBuf;
use walkdir::WalkDir;

pub fn merge_file_paths(sync_path: Vec<PathBuf>) -> Vec<PathBuf> {
    let all_paths: Vec<PathBuf> = {
        let mut empty_all_paths = Vec::<PathBuf>::new();
        for path in sync_path {
            empty_all_paths.append(&mut get_file_paths_recursive(&path));
        }
        std::mem::take(&mut empty_all_paths)
    };
    all_paths
}

fn get_file_paths_recursive(import_path: &PathBuf) -> Vec<PathBuf> {
    WalkDir::new(import_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|dir_entry| dir_entry.path().is_file())
        .map(|dir_entry| dir_entry.into_path())
        .collect()
}
