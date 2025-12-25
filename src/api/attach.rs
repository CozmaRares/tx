use crate::commands::tmux;

pub fn handle_attach() -> anyhow::Result<()> {
   tmux::attach_to_last_session()
}
