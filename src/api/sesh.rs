use crate::{
    commands::{fzf, tmux::TmuxSessionBuilder},
    data::{TxDirectory, TxLayout},
};

pub fn handle_sesh(layout: Option<String>) -> anyhow::Result<()> {
    let dirs = TxDirectory::get_all()?;
    let data = dirs
        .iter()
        .map(|d| d.get_last_2_parts())
        .collect::<Vec<_>>()
        .join("\n");
    let selected = fzf::pick_dir(&data)?;

    let Some(dir) = TxDirectory::find(&selected) else {
        anyhow::bail!("Directory not found");
    };

    if let Some(layout) = layout {
        let Some(mut layout) = TxLayout::find(&layout) else {
            anyhow::bail!("Layout not found");
        };
        layout.override_root(dir.path);
        layout.open()
    } else {
        let builder = TmuxSessionBuilder::new_from_dir(dir.path);
        builder.create_session()?;
        builder.open_session()
    }
}
