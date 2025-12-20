mod api;
mod cli;
mod commands;
mod data;
mod managers;

use crate::{
    api::{edit::handle_edit, ls::handle_ls, rml::handle_rml, session::handle_new_session},
    cli::{Cli, Command},
    managers::DataDirManager,
};
use clap::Parser;

fn setup() -> anyhow::Result<()> {
    commands::ensure_deps()?;
    DataDirManager::ensure_tx_dir_structure()?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    setup()?;

    let cli = Cli::parse();

    #[cfg(debug_assertions)]
    println!("{:#?}", cli);

    if let Some(session_name) = cli.session_name {
        return handle_new_session(session_name);
    } else if cli.command.is_none() {
        todo!("Attach to last session");
    }

    let command = cli.command.unwrap();

    match command {
        Command::Ls { all } => handle_ls(all),
        Command::Edit(args) => handle_edit(args),
        Command::Rml { name } => handle_rml(name),
        Command::Pick => todo!("Pick a session"),
        Command::Switch => todo!("Switch to last session"),
        Command::Sesh => todo!("Create session from dirs"),
        Command::Dot => todo!("Create session from cwd"),
    }
}
