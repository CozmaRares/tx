use std::{fs, thread};

use crate::{commands::eza, managers::DirsManager};

#[derive(Debug)]
pub struct TxDirectory {
    pub path: String,
    last_2_parts_start: usize,
}

impl TxDirectory {
    pub fn new(dir: String) -> Self {
        let second_to_last_slash_idx = dir.rmatch_indices('/').nth(1).map(|(idx, _)| idx);

        if let Some(second_to_last_slash_idx) = second_to_last_slash_idx {
            TxDirectory {
                path: dir,
                last_2_parts_start: second_to_last_slash_idx + 1,
            }
        } else {
            TxDirectory {
                path: dir,
                last_2_parts_start: 0,
            }
        }
    }

    pub fn get_all() -> anyhow::Result<Vec<TxDirectory>> {
        let top_level_dirs = DirsManager::get_dir_paths()?;
        let handles: Vec<_> = top_level_dirs
            .into_iter()
            .map(|dir| {
                thread::spawn(move || {
                    fs::read_dir(&dir)
                        .unwrap()
                        .into_iter()
                        .filter_map(|result| result.ok())
                        .filter(|entry| entry.path().is_dir())
                        .map(|entry| entry.path().to_string_lossy().to_string())
                        .collect::<Vec<_>>()
                })
            })
            .collect();

        let mut all_dirs: Vec<String> = handles
            .into_iter()
            .flat_map(|h| h.join().unwrap_or_default())
            .collect();

        all_dirs.sort();
        all_dirs.dedup();

        Ok(all_dirs.into_iter().map(TxDirectory::new).collect())
    }

    pub fn get_last_2_parts(&self) -> &str {
        &self.path[self.last_2_parts_start..]
    }

    pub fn preview(&self) -> anyhow::Result<()> {
        eza::preview_dir(&self.path)
    }

    pub fn find(dir: &str) -> Option<TxDirectory> {
        let Ok(dirs) = TxDirectory::get_all() else {
            return None;
        };
        dirs.into_iter().find(|d| d.get_last_2_parts() == dir)
    }
}
