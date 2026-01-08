use crate::commands::tmux::TmuxSessionBuilder;

pub fn handle_dot() -> anyhow::Result<()> {
    let builder = TmuxSessionBuilder::new(None, None);
    builder.create_session()?;
    builder.open_session()
}
