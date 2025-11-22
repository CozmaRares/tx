use std::cmp::Ordering;

use anyhow::Result;

use crate::runner::{
    fs::{TxDirectory, TxFragment, TxLayout},
    tmux::TmuxSession,
};

pub enum TxData {
    Session(TmuxSession),
    Layout(TxLayout),
    Fragment(TxFragment),
    Directory(TxDirectory),
}

impl TxData {
    fn len(&self) -> usize {
        match self {
            TxData::Session(TmuxSession { name, .. }) => name.len(),
            TxData::Layout(TxLayout(name)) => name.len(),
            TxData::Fragment(TxFragment(name)) => name.len(),
            TxData::Directory(dir) => dir.get_last_2_parts().len(),
        }
    }

    fn to_string(&self, spaces: usize) -> String {
        match self {
            TxData::Session(session) => format!(
                "{:<spaces$} (session) ({} windows){}",
                session.name,
                session.num_windows,
                if session.is_attached {
                    " (attached)"
                } else {
                    ""
                },
                spaces = spaces
            ),
            TxData::Layout(layout) => format!("{:<spaces$} (layout)", layout.0, spaces = spaces),
            TxData::Fragment(fragment) => {
                format!("{:<spaces$} (fragment)", fragment.0, spaces = spaces)
            }
            TxData::Directory(dir) => format!(
                "{:<spaces$} (directory)",
                dir.get_last_2_parts(),
                spaces = spaces
            ),
        }
    }

    pub fn open(self) -> Result<()> {
        match self {
            TxData::Session(session) => TmuxSession::open(&session.name),
            TxData::Layout(layout) => TxLayout::open(&layout.0),
            TxData::Fragment(fragment) => TxFragment::open(&fragment.0),
            TxData::Directory(dir) => TxDirectory::open(dir.get_last_2_parts()),
        }
    }
}

pub fn list(all: bool) -> Result<Vec<TxData>> {
    let mut sessions = TmuxSession::get_all()?;
    sessions.sort_by(|a, b| match a.is_attached.cmp(&b.is_attached) {
        Ordering::Equal => b.last_attached.cmp(&a.last_attached),
        other => other,
    });

    let layouts = TxLayout::get_all()?;
    let fragments = TxFragment::get_all()?;

    let dirs = if all {
        TxDirectory::get_all()?
    } else {
        Vec::new()
    };

    let mut data = Vec::new();
    data.extend(sessions.into_iter().map(TxData::Session));
    data.extend(layouts.into_iter().map(TxData::Layout));
    data.extend(fragments.into_iter().map(TxData::Fragment));
    data.extend(dirs.into_iter().map(TxData::Directory));

    Ok(data)
}

pub fn data_to_string(data: &[TxData]) -> String {
    let max_len = data.iter().map(|d| d.len()).max().unwrap_or(0);
    data.iter()
        .map(|d| d.to_string(max_len))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn handle_list(all: bool) -> Result<()> {
    let data = list(all)?;
    println!("{}", data_to_string(&data));
    Ok(())
}
