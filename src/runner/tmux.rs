use std::env;

use anyhow::Result;

use super::utils::{execvp, run_command};

pub const DEPS: &[&str] = &["tmux"];

#[derive(Debug, Clone)]
pub struct TmuxSession {
    pub name: String,
    pub is_attached: bool,
    pub last_attached: usize,
    pub num_windows: usize,
}

impl TmuxSession {
    pub fn get_all() -> Result<Vec<TmuxSession>> {
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

    pub fn open(self) -> Result<()> {
        let cmd = if env::var("TMUX").is_err() {
            vec!["tmux", "attach-session", "-t", &self.name]
        } else {
            vec!["tmux", "switch-client", "-t", &self.name]
        };
        execvp(&cmd)
    }

    pub fn preview(&self) -> Result<()> {
        let cmd = vec!["tmux", "capture-pane", "-ep", "-t", &self.name];
        execvp(&cmd)
    }
}
