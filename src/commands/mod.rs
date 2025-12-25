pub mod bat;
pub mod eza;
pub mod fzf;
pub mod runner;
pub mod tmux;

use std::{env, fs};

pub fn ensure_deps() -> anyhow::Result<()> {
    let deps = [bat::PROGRAM, eza::PROGRAM, fzf::PROGRAM, tmux::PROGRAM];

    for dep in deps {
        let dep = dep.to_string();
        if !is_executable_in_path(&dep) {
            anyhow::bail!("Required program not found in PATH: `{}`", dep);
        }
    }

    Ok(())
}

fn is_executable_in_path(cmd: &str) -> bool {
    if let Some(paths) = env::var_os("PATH") {
        for path in env::split_paths(&paths) {
            let full_path = path.join(cmd);
            if full_path.exists()
                && fs::metadata(&full_path)
                    .map(|m| m.is_file())
                    .unwrap_or(false)
            {
                return true;
            }
        }
    }
    false
}
