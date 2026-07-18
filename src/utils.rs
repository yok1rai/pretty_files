use walkdir::WalkDir;
use std::path::{Path, PathBuf};

pub fn recursive_search(root: impl AsRef<Path>, ignored: &[String]) -> Vec<PathBuf> {
        WalkDir::new(root)
            .into_iter()
            .filter_entry(|entry| {
                let path = entry.path().to_string_lossy();
                !ignored.iter().any(|i| path.ends_with(i))
            })
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
            .map(|entry| entry.into_path())
            .collect()
}
