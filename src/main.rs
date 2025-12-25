mod api;
mod cli;
mod commands;
mod data;
mod managers;

use clap::Parser;

use crate::{
    api::{
        handle_attach, handle_edit, handle_ls, handle_new_session, handle_pick, handle_preview,
        handle_rml, handle_switch,
    },
    cli::{Cli, Command},
    managers::DataDirManager,
};

fn setup() -> anyhow::Result<()> {
    commands::ensure_deps()?;
    DataDirManager::ensure_tx_dir_structure()?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    setup()?;

    let cli = Cli::parse();

    if let Some(session_name) = cli.session_name {
        return handle_new_session(session_name);
    } else if cli.command.is_none() {
        return handle_attach();
    }

    let command = cli.command.unwrap();

    match command {
        Command::Ls { all } => handle_ls(all),
        Command::Edit(args) => handle_edit(args),
        Command::Rml { name } => handle_rml(name),
        Command::Preview(args) => handle_preview(args),
        Command::Pick => handle_pick(),
        Command::Switch => handle_switch(),
        Command::Sesh => todo!("Create session from dirs"),
        Command::Dot => todo!("Create session from cwd"),
    }
}
