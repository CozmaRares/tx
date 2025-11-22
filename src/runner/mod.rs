mod utils;
pub use utils::execvp;

pub mod bat;
pub mod eza;
pub mod fs;
pub mod fzf;
pub mod tmux;

use anyhow::{Context, Result};

pub fn ensure_dependencies() -> Result<()> {
    let fs_deps = fs::DEPS;
    let bat_deps = bat::DEPS;
    let eza_deps = eza::DEPS;
    let fzf_deps = fzf::DEPS;
    let tmux_deps = tmux::DEPS;

    let all_deps: Vec<_> = fs_deps
        .iter()
        .chain(bat_deps)
        .chain(eza_deps)
        .chain(fzf_deps)
        .chain(tmux_deps)
        .collect();

    utils::ensure_deps(&all_deps).context("Not all dependencies found")
}
