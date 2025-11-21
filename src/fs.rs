use std::thread;

use anyhow::Result;

use crate::runner::run_command;

pub const TX_DIR: &str = "/home/raru/.tx";
const DATA_DIR: &str = "/home/raru/.tx/data";
const DIRS_FILE: &str = "/home/raru/.tx/data/dirs.txt";

const LAYOUT_EXT: &str = ".layout.sh";
const FRAGMENT_EXT: &str = ".fragment.sh";

pub fn get_layouts() -> Result<Vec<String>> {
    ensure_data_dir()?;
    run_command(&[
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
    .map(split_lines)
}

pub fn get_fragments() -> Result<Vec<String>> {
    ensure_data_dir()?;
    run_command(&[
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
    .map(split_lines)
}

pub fn get_dir_paths() -> Result<Vec<String>> {
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

    Ok(all_dirs)
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
