use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn get_toml_files_in_dir(
    dir_path: &Path,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let entries = fs::read_dir(dir_path)?
        .filter_map(|entry| entry.ok().map(|e| e.path())) // Convert DirEntry to PathBuf and filter out errors
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("toml")) // Keep only TOML files
        .collect::<Vec<PathBuf>>();

    Ok(entries)
}
