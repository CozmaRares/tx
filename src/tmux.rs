use std::env;

use anyhow::Result;

use crate::runner::{execvp, run_command};

#[derive(Debug, Clone)]
pub struct TmuxSession {
    pub name: String,
    pub is_attached: bool,
    pub last_attached: usize,
    pub num_windows: usize,
}

pub fn get_tmux_sessions() -> Result<Vec<TmuxSession>> {
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

pub fn open_session(session: &str) -> Result<()> {
    let cmd = if env::var("TMUX").is_err() {
        vec!["tmux", "attach-session", "-t", session]
    } else {
        vec!["tmux", "switch-client", "-t", session]
    };

    execvp(&cmd)
}
