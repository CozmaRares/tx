use std::thread;

use anyhow::Result;

use super::{
    bat, eza,
    utils::{execvp, run_command},
};

pub const DEPS: &[&str] = &["find", "cat", "test", "mkdir", "touch"];

pub const TX_DIR: &str = "/home/raru/.tx";
const DATA_DIR: &str = "/home/raru/.tx/data";
const DIRS_FILE: &str = "/home/raru/.tx/data/dirs.txt";

const LAYOUT_EXT: &str = ".layout.sh";
const FRAGMENT_EXT: &str = ".fragment.sh";

#[derive(Debug, Clone)]
pub struct TxLayout(pub String);

impl TxLayout {
    pub fn get_all() -> Result<Vec<TxLayout>> {
        ensure_data_dir()?;
        let lines = run_command(&[
            "find",
            DATA_DIR,
            "-type",
            "f",
            "-name",
            &format!("*{}", LAYOUT_EXT),
            "-exec",
            "basename",
            "{}",
            LAYOUT_EXT,
            ";",
        ])
        .map(split_lines)?;
        Ok(lines.into_iter().map(TxLayout).collect())
    }

    pub fn preview(&self) -> Result<()> {
        bat::preview(format!("{}/{}{}", DATA_DIR, self.0, LAYOUT_EXT))
    }
}

#[derive(Debug, Clone)]
pub struct TxFragment(pub String);

impl TxFragment {
    pub fn get_all() -> Result<Vec<TxFragment>> {
        ensure_data_dir()?;
        let lines = run_command(&[
            "find",
            DATA_DIR,
            "-type",
            "f",
            "-name",
            &format!("*{}", FRAGMENT_EXT),
            "-exec",
            "basename",
            "{}",
            FRAGMENT_EXT,
            ";",
        ])
        .map(split_lines)?;
        Ok(lines.into_iter().map(TxFragment).collect())
    }

    pub fn preview(&self) -> Result<()> {
        bat::preview(format!("{}/{}{}", DATA_DIR, self.0, FRAGMENT_EXT))
    }
}

#[derive(Debug, Clone)]
pub struct TxDirectory {
    pub name: String,
    pub last_2_parts_start: usize,
}

impl TxDirectory {
    pub fn get_all() -> Result<Vec<TxDirectory>> {
        ensure_dirs_file()?;
        let top_level_dirs = run_command(&["cat", DIRS_FILE]).map(split_lines)?;

        let handles: Vec<_> = top_level_dirs
            .into_iter()
            .map(|dir| {
                thread::spawn(move || {
                    let mut dirs = Vec::new();
                    if let Ok(output) = run_command(&[
                        "find",
                        &dir,
                        "-mindepth",
                        "1",
                        "-maxdepth",
                        "1",
                        "-type",
                        "d",
                    ]) {
                        let nested_dirs = split_lines(output);
                        dirs.extend(nested_dirs);
                    }
                    dirs
                })
            })
            .collect();

        let mut all_dirs: Vec<String> = handles
            .into_iter()
            .flat_map(|h| h.join().unwrap_or_default())
            .collect();

        all_dirs.sort();
        all_dirs.dedup();

        Ok(all_dirs
            .into_iter()
            .map(|dir| {
                let second_to_last_slash_idx = dir.rmatch_indices('/').nth(1).map(|(idx, _)| idx);

                if let Some(second_to_last_slash_idx) = second_to_last_slash_idx {
                    TxDirectory {
                        name: dir,
                        last_2_parts_start: second_to_last_slash_idx + 1,
                    }
                } else {
                    TxDirectory {
                        name: dir,
                        last_2_parts_start: 0,
                    }
                }
            })
            .collect())
    }

    pub fn get_last_2_parts(&self) -> &str {
        &self.name[self.last_2_parts_start..]
    }

    pub fn preview(&self) -> Result<()> {
        eza::preview(&self.name)
    }
}

fn ensure_data_dir() -> Result<()> {
    let exists = run_command(&["test", "-d", DATA_DIR]).is_ok();
    if !exists {
        run_command(&["mkdir", "-p", DATA_DIR])?;
    }
    Ok(())
}

fn ensure_dirs_file() -> Result<()> {
    ensure_data_dir()?;
    let exists = run_command(&["test", "-f", DIRS_FILE]).is_ok();
    if !exists {
        run_command(&["touch", DIRS_FILE])?;
    }
    Ok(())
}

fn split_lines(s: String) -> Vec<String> {
    s.lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect()
}
