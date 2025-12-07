use super::runner::{execvp, run_command};
use crate::data::TmuxSession;
use std::env;

pub(super) const PROGRAM: &str = "tmux";

pub fn get_sessions() -> anyhow::Result<Vec<TmuxSession>> {
    run_command(&[
        "tmux",
        "ls",
        "-F",
        "#{session_name}:#{session_attached}:#{session_last_attached}:#{session_windows}",
    ])
    .map(|output| {
        output
            .lines()
            .map(|line| {
                let mut parts = line.split(':');
                let name = parts.next().unwrap();
                let is_attached = parts.next().map(|s| s == "1").unwrap();
                let last_attached = parts.next().map(|s| s.parse::<usize>().unwrap()).unwrap();
                let num_windows = parts.next().map(|s| s.parse::<usize>().unwrap()).unwrap();

                TmuxSession {
                    name: name.to_string(),
                    is_attached,
                    last_attached,
                    num_windows,
                }
            })
            .collect()
    })
}

pub fn open_session(name: &str) -> anyhow::Result<()> {
    let cmd = if env::var("TMUX").is_err() {
        vec!["tmux", "attach-session", "-t", name]
    } else {
        vec!["tmux", "switch-client", "-t", name]
    };
    execvp(&cmd)
}

pub fn preview_pane(name: &str) -> anyhow::Result<()> {
    let cmd = vec!["tmux", "capture-pane", "-ep", "-t", name];
    execvp(&cmd)
}
