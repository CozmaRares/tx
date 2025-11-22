use anyhow::{bail, Result};

use crate::{
    cli::PreviewKind,
    runner::{
        fs::{TxDirectory, TxFragment, TxLayout},
        tmux::TmuxSession,
    },
};

pub fn handle_preview(kind: PreviewKind, value: String) -> Result<()> {
    match kind {
        PreviewKind::Session => preview_session(value),
        PreviewKind::Layout => preview_layout(value),
        PreviewKind::Fragment => preview_fragment(value),
        PreviewKind::Directory => preview_directory(value),
    }
}

fn preview_session(name: String) -> Result<()> {
    let sessions = TmuxSession::get_all()?;
    println!("{:?}", sessions);
    if let Some(session) = sessions.iter().find(|s| s.name == name) {
        session.preview()
    } else {
        bail!("Session '{}' not found", name);
    }
}

fn preview_layout(name: String) -> Result<()> {
    let layouts = TxLayout::get_all()?;
    if let Some(layout) = layouts.iter().find(|l| l.0 == name) {
        layout.preview()
    } else {
        bail!("Layout '{}' not found", name);
    }
}

fn preview_fragment(name: String) -> Result<()> {
    let fragments = TxFragment::get_all()?;
    if let Some(fragment) = fragments.iter().find(|f| f.0 == name) {
        fragment.preview()
    } else {
        bail!("Fragment '{}' not found", name);
    }
}

fn preview_directory(name: String) -> Result<()> {
    let dirs = TxDirectory::get_all()?;
    if let Some((dir, _)) = dirs
        .iter()
        .map(|d| (d, d.get_last_2_parts()))
        .find(|(_, parts)| *parts == name)
    {
        dir.preview()
    } else {
        bail!("Directory '{}' not found", name);
    }
}
