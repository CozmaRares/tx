use anyhow::Result;

use super::utils::execvp;

pub const DEPS: &[&str] = &["bat"];

pub fn preview(path: String) -> Result<()> {
    let cmd = vec![
        "bat",
        "--paging=never",
        "--style=plain",
        "--color=always",
        &path,
    ];
    execvp(&cmd)
}
