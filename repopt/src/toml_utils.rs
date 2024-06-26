use std::fs;
use std::path::{Path, PathBuf};

type Error = Box<dyn std::error::Error>; // replace this with set error types for production code.
type Result<T> = std::result::Result<T, Error>;

pub(crate) fn get_toml_files_in_dir(dir_path: &Path) -> Result<Vec<PathBuf>> {
    let entries = fs::read_dir(dir_path)?
        .filter_map(|entry| entry.ok().map(|e| e.path())) // Convert DirEntry to PathBuf and filter out errors
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("toml")) // Keep only TOML files
        .collect::<Vec<PathBuf>>();

    Ok(entries)
}
