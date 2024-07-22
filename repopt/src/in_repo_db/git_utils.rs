use std::path::{Path, PathBuf};

type Error = Box<dyn std::error::Error>; // replace this with set error types for production code.
type Result<T> = std::result::Result<T, Error>;

pub(super) fn find_git_root<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let repo = gix::discover(path)?;
    Ok(repo
        .git_dir()
        .parent()
        .ok_or("Git root not found")?
        .to_path_buf()
        .canonicalize()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_git_root() {
        let repo = find_git_root(".").unwrap();
        assert_eq!(repo, Path::new("../").canonicalize().unwrap());
    }
}
