use crate::commands::runner::execvp;
use anyhow::Result;

pub(super) const PROGRAM: &str = "bat";

pub fn preview_file(path: String) -> Result<()> {
    let cmd = vec![
        "bat",
        "--paging=never",
        "--style=plain",
        "--color=always",
        &path,
    ];
    execvp(&cmd)
}
