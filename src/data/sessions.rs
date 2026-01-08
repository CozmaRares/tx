use crate::commands::tmux;

#[derive(Debug)]
pub struct TmuxSession {
    pub name: String,
    pub is_attached: bool,
    pub last_attached: usize,
    pub num_windows: usize,
}

impl TmuxSession {
    pub fn get_all() -> Vec<TmuxSession> {
        tmux::get_sessions()
    }

    pub fn open(name: &str) -> anyhow::Result<()> {
        tmux::open_session(name)
    }

    pub fn preview(name: &str) -> anyhow::Result<()> {
        tmux::preview_pane(name)
    }
}
