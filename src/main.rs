mod api;
mod cli;
mod commands;
mod data;
mod managers;

use crate::{
    api::{edit::handle_edit, ls::handle_ls, rml::handle_rml},
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

    println!("{:#?}", cli);

    match cli.command {
        Some(Command::Ls { all }) => handle_ls(all),
        Some(Command::Edit(args)) => handle_edit(args),
        Some(Command::Rml { name }) => handle_rml(name),
        None => {
            todo!("Attach to last session");
        }
    }
}
