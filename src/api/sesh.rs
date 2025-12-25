use crate::{commands::fzf, data::TxDirectory};

pub fn handle_sesh() -> anyhow::Result<()> {
    let dirs = TxDirectory::get_all()?;
    let data = dirs
        .iter()
        .map(|d| d.get_last_2_parts())
        .collect::<Vec<_>>()
        .join("\n");
    let selected = fzf::pick_dir(&data)?;

    TxDirectory::find(&selected)?.open()
}
