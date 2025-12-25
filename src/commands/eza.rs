use crate::commands::runner::execvp;

pub(super) const PROGRAM: &str = "eza";

pub fn preview_dir(path: &str) -> anyhow::Result<()> {
    let cmd = vec!["eza", "--group-directories-first", "-lah", path];
    execvp(&cmd)
}
