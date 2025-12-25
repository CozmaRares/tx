use std::process::exit;

use crate::{
    api::ls::{self, ListFilter, data_to_string},
    commands::fzf,
    data::{TmuxSession, TxDirectory, TxLayout},
};

pub fn handle_pick() -> anyhow::Result<()> {
    let data = ls::list(ListFilter::Regular)?;
    let data = data_to_string(data);
    let selected = fzf::pick_session(&data)?;

    if selected.is_empty() {
        exit(1);
    }

    let split: Vec<&str> = selected.split(' ').filter(|s| !s.is_empty()).collect();
    let name = split[0];
    let r#type = split[1];

    match r#type {
        "(session)" => TmuxSession::open(name),
        "(layout)" => TxLayout::new(name.to_string()).open(),
        "(directory)" => TxDirectory::new(name.to_string()).open(),
        _ => anyhow::bail!("Unknown type: {}", r#type),
    }
}
