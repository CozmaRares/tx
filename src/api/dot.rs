use crate::commands::tmux;

pub fn handle_dot() -> anyhow::Result<()> {
    tmux::new_session()
}
