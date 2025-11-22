use anyhow::Result;

use super::utils::run_command_with_stdin;

pub const DEPS: &[&str] = &["fzf"];

pub fn picker(input: &str, preview_cmd: &str) -> Result<String> {
    let cmd = vec![
        "fzf",
        "--color=dark,gutter:-1",
        "--cycle",
        "--tmux",
        "center,75%,80%",
        "--reverse",
        "--bind",
        "tab:down",
        "--bind",
        "btab:up",
        "--preview",
        preview_cmd,
        "--preview-window",
        "up,75%,border-bottom",
    ];

    run_command_with_stdin(&cmd, input)
}
