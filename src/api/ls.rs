use std::cmp::Ordering;

use crate::data::{TmuxSession, TxDirectory, TxLayout};

pub enum LsData {
    Session(TmuxSession),
    Layout(TxLayout),
    Directory(TxDirectory),
}

impl LsData {
    fn len(&self) -> usize {
        match self {
            LsData::Session(TmuxSession { name, .. }) => name.len(),
            LsData::Layout(TxLayout { name, .. }) => name.len(),
            LsData::Directory(dir) => dir.get_last_2_parts().len(),
        }
    }

    pub fn to_string(&self, spaces: usize) -> String {
        match self {
            LsData::Session(session) => format!(
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
            LsData::Layout(layout) => format!("{:<spaces$} (layout)", layout.name, spaces = spaces),
            LsData::Directory(dir) => format!(
                "{:<spaces$} (directory)",
                dir.get_last_2_parts(),
                spaces = spaces
            ),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum ListFilter {
    JustSessions,
    Regular,
    All,
}

pub fn list(filter: ListFilter) -> anyhow::Result<Vec<LsData>> {
    let mut sessions = TmuxSession::get_all();
    sessions.sort_by(|a, b| match a.is_attached.cmp(&b.is_attached) {
        Ordering::Equal => b.last_attached.cmp(&a.last_attached),
        other => other,
    });

    let mut data: Vec<_> = sessions.into_iter().map(LsData::Session).collect();

    if filter >= ListFilter::Regular {
        let layouts = TxLayout::get_all()?;
        data.extend(layouts.into_iter().map(LsData::Layout));
    }

    if filter >= ListFilter::All {
        let dirs = TxDirectory::get_all()?;
        data.extend(dirs.into_iter().map(LsData::Directory));
    }

    Ok(data)
}

pub fn data_to_string(data: Vec<LsData>) -> String {
    let max_len = data.iter().map(|d| d.len()).max().unwrap_or(0);
    data.iter()
        .map(|d| d.to_string(max_len))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn handle_ls(all: bool) -> anyhow::Result<()> {
    let data = list(if all {
        ListFilter::All
    } else {
        ListFilter::Regular
    })?;
    println!("{}", data_to_string(data));
    Ok(())
}
