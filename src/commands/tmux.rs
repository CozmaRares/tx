use std::env;

use crate::{
    commands::runner::{execvp, run_command},
    data::TmuxSession,
};

pub(super) const PROGRAM: &str = "tmux";

pub fn get_sessions() -> Vec<TmuxSession> {
    run_command(&[
        "tmux",
        "ls",
        "-F",
        "#{session_name}:#{session_attached}:#{session_windows}:#{session_last_attached}",
    ])
    .map(|output| {
        output
            .lines()
            .map(|line| {
                let mut parts = line.split(':');
                let name = parts.next().unwrap();
                let is_attached = parts.next().map(|s| s == "1").unwrap();
                let num_windows = parts.next().map(|s| s.parse::<usize>().unwrap()).unwrap();
                let last_attached = parts
                    .next()
                    .map(|s| {
                        if s.len() == 0 {
                            usize::MAX
                        } else {
                            s.parse::<usize>().unwrap()
                        }
                    })
                    .unwrap();

                TmuxSession {
                    name: name.to_string(),
                    is_attached,
                    last_attached,
                    num_windows,
                }
            })
            .collect()
    })
    .unwrap_or_default()
}

pub fn open_session(name: &str) -> anyhow::Result<()> {
    let cmd = if env::var("TMUX").is_err() {
        vec!["tmux", "attach-session", "-t", name]
    } else {
        vec!["tmux", "switch-client", "-t", name]
    };
    execvp(&cmd)
}

pub fn attach_to_last_session() -> anyhow::Result<()> {
    execvp(&["tmux", "attach-session"])
}

pub fn preview_pane(name: &str) -> anyhow::Result<()> {
    execvp(&["tmux", "capture-pane", "-ep", "-t", name])
}

pub struct TmuxSessionBuilder {
    session_name: String,
    session_root: String,
    current_window: String,
    current_pane: usize,
}

macro_rules! format_pane {
    ($session:expr) => {
        &format!("{}", $session)
    };
    ($session:expr, $winow:expr) => {
        &format!("{}:{}", $session, $winow)
    };
    ($session:expr, $winow:expr, $pane:expr) => {
        &format!("{}:{}.{}", $session, $winow, $pane)
    };
}

impl TmuxSessionBuilder {
    pub fn new(name: &str, root: Option<String>) -> Self {
        Self {
            session_name: name.to_string(),
            session_root: root.unwrap_or(
                std::env::current_dir()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            ),

            current_window: String::new(),
            current_pane: 1,
        }
    }

    fn set_window(&mut self, name: &str) {
        self.current_window = name.to_string();
        self.set_pane(1);
    }

    fn set_pane(&mut self, pane: usize) {
        self.current_pane = pane;
    }

    pub fn create_session(&self) -> anyhow::Result<()> {
        run_command(&[
            "tmux",
            "new-session",
            "-d",
            "-s",
            &self.session_name,
            "-c",
            &self.session_root,
        ])
        .map(|_s| {})
    }

    pub fn create_window(&mut self, name: &str) -> anyhow::Result<()> {
        run_command(&[
            "tmux",
            "new-window",
            "-t",
            format_pane!(self.session_name),
            "-c",
            &self.session_root,
            "-n",
            name,
        ])?;
        self.set_window(name);
        Ok(())
    }

    pub fn split_pane(&mut self, direction: &str, size: usize) -> anyhow::Result<()> {
        if direction != "-h" && direction != "-v" {
            anyhow::bail!("Invalid pane direction: {}", direction)
        };

        run_command(&[
            "tmux",
            "split-window",
            "-t",
            format_pane!(self.session_name, self.current_window, self.current_pane),
            "-c",
            &self.session_root,
            direction,
            "-l",
            &format!("{}%", size),
        ])?;

        self.set_pane(self.current_pane + 1);
        Ok(())
    }

    pub fn select_window(&mut self, window: &str) -> anyhow::Result<()> {
        run_command(&[
            "tmux",
            "select-window",
            "-t",
            format_pane!(self.session_name, window),
        ])?;
        self.set_window(window);
        Ok(())
    }

    pub fn select_pane(&mut self, pane: usize) -> anyhow::Result<()> {
        run_command(&[
            "tmux",
            "select-pane",
            "-t",
            format_pane!(self.session_name, self.current_window, pane),
        ])?;
        self.set_pane(pane);
        Ok(())
    }

    pub fn run_command(&mut self, command: &str) -> anyhow::Result<()> {
        run_command(&[
            "tmux",
            "send-keys",
            "-t",
            format_pane!(self.session_name, self.current_window, self.current_pane),
            command,
            "C-m",
        ])
        .map(|_s| {})
    }

    pub fn open_session(self) -> anyhow::Result<()> {
        open_session(&self.session_name)
    }

    pub fn rename_window(&mut self, name: &str) -> anyhow::Result<()> {
        run_command(&[
            "tmux",
            "rename-window",
            "-t",
            format_pane!(self.session_name, self.current_window),
            name,
        ])?;
        self.set_window(name);
        Ok(())
    }
}
