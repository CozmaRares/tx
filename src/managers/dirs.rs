use std::fs;

use crate::managers::common::{TX_ROOT, ensure_file_exists};

const DIRS_FILE: &str = "dirs.txt";

pub struct DirsManager;

impl DirsManager {
    pub fn get_dirs_file() -> String {
        format!("{}/{}", TX_ROOT.as_str(), DIRS_FILE)
    }

    pub fn ensure_dirs_file() -> anyhow::Result<()> {
        ensure_file_exists(&DirsManager::get_dirs_file(), || "".to_string())
    }

    pub fn get_dir_paths() -> anyhow::Result<Vec<String>> {
        let file = format!("{}/{}", TX_ROOT.as_str(), DIRS_FILE);
        let contents = fs::read_to_string(file)?;
        Ok(contents
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect())
    }
}
