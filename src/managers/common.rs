use std::{fs, path::Path};

lazy_static::lazy_static! {
    pub static ref TX_ROOT: String = std::env::var("TX_ROOT").expect("Env var TX_ROOT not set");
}

pub fn ensure_dir_exists(dir: &str) -> anyhow::Result<()> {
    let path = Path::new(dir);

    if !path.exists() {
        fs::create_dir(path)?
    } else if !path.is_dir() {
        anyhow::bail!("{} is not a directory", dir);
    }

    Ok(())
}

pub fn ensure_file_exists(file: &str, contents: impl FnOnce() -> String) -> anyhow::Result<()> {
    let path = Path::new(file);

    if !path.exists() {
        fs::write(path, contents())?;
    } else if !path.is_file() {
        anyhow::bail!("{} is not a file", file);
    }

    Ok(())
}
