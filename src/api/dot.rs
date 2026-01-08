use crate::commands::tmux::TmuxSessionBuilder;

pub fn handle_dot() -> anyhow::Result<()> {
    let builder = TmuxSessionBuilder::default();
    builder.create_session()?;
    builder.open_session()
}
