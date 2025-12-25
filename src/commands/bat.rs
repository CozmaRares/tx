use crate::commands::runner::execvp;

pub(super) const PROGRAM: &str = "bat";

pub fn preview_file(path: String) -> anyhow::Result<()> {
    let cmd = vec![
        "bat",
        "--paging=never",
        "--style=plain",
        "--color=always",
        &path,
    ];
    execvp(&cmd)
}
