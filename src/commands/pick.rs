use std::process::exit;

use anyhow::{bail, Result};

use crate::runner::{
    fs::{TxDirectory, TxFragment, TxLayout},
    fzf,
    tmux::TmuxSession,
};

use super::list::{data_to_string, list};

pub fn handle_pick() -> Result<()> {
    let preview_cmd = r#"echo {} | awk '{
        match($0, /\(([^)]+)\)/, arr);
        type = substr(arr[1], 1, 1);
        gsub(/\(.*\)/, "", $0);
        name = $0
        gsub(/[[:space:]]+$/, "", name)
        printf "tx preview -%s \"%s\"", type, name
    }' | bash"#;

    let data = list(false)?;
    let data = data_to_string(&data);
    let selected = fzf::picker(&data, &preview_cmd)?;

    if selected.is_empty() {
        exit(1);
    }

    open_selection(&selected)
}

pub fn open_selection(selected: &str) -> Result<()> {
    let split: Vec<&str> = selected.split(' ').filter(|s| !s.is_empty()).collect();
    let name = split[0];
    let r#type = split[1];

    match r#type {
        "(session)" => TmuxSession::open(name),
        "(layout)" => TxLayout::open(name),
        "(fragment)" => TxFragment::open(name),
        "(directory)" => TxDirectory::open(name),
        _ => bail!("Unknown type: {}", r#type),
    }
}
