use crate::commands::tmux;

#[derive(Debug, Clone)]
pub struct TmuxSession {
    pub name: String,
    pub is_attached: bool,
    pub last_attached: usize,
    pub num_windows: usize,
}

impl TmuxSession {
    pub fn get_all() -> anyhow::Result<Vec<TmuxSession>> {
        tmux::get_sessions()
    }

    pub fn open(name: &str) -> anyhow::Result<()> {
        tmux::open_session(name)
    }

    pub fn preview(&self) -> anyhow::Result<()> {
        tmux::preview_pane(&self.name)
    }
}
