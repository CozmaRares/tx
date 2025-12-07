use crate::commands::runner::execvp;
use anyhow::Result;

pub(super) const PROGRAM: &str = "eza";

pub fn preview_dir(path: &str) -> Result<()> {
    let cmd = vec!["eza", "--group-directories-first", "-lah", path];
    execvp(&cmd)
}
