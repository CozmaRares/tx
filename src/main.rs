// Credits to
// - tmuxifier: https://github.com/jimeh/tmuxifier
// - shmux: https://github.com/typecraft-dev/shmux
// - ThePrimeagen's sessionizer script: https://github.com/ThePrimeagen/.dotfiles/

mod commands;
mod cli;
mod runner;

use anyhow::Result;

use crate::{commands::*, cli::Command};

fn main() -> Result<()> {
    runner::ensure_dependencies()?;

    let command = cli::parse_args()?;

    match command {
        Command::Help => help::handle_help(),
        Command::Ls { all } => list::handle_list(all),
        Command::Preview { kind, value } => preview::handle_preview(kind, value),
        Command::Edit(resource) => edit::handle_edit(resource),
        Command::Pick => pick::handle_pick(),
        Command::Switch => todo!(),
        Command::Sesh => todo!(),
        Command::NewSession(session_location) => todo!(),
        Command::Attach => todo!(),
    }
}
