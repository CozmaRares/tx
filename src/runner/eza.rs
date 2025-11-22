use anyhow::Result;

use super::utils::execvp;

pub const DEPS: &[&str] = &["eza"];

pub fn preview(path: &str) -> Result<()> {
    let cmd = vec!["eza", "--group-directories-first", "-lah", path];
    execvp(&cmd)
}
