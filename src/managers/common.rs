use std::{fs, path::Path};

pub const TX_DIR: &str = "/tmp/.tx";

pub fn ensure_dir_exists(dir: &str) -> anyhow::Result<()> {
    let path = Path::new(dir);

    if !path.exists() {
        fs::create_dir(path)?
    } else if !path.is_dir() {
        anyhow::bail!("{} is not a directory", dir);
    }

    Ok(())
}

pub fn ensure_file_exists(
    file: &str,
    contents: Option<impl FnOnce() -> String>,
) -> anyhow::Result<()> {
    let path = Path::new(file);

    if !path.exists() {
        if let Some(contents) = contents {
            fs::write(path, contents())?;
        } else {
            anyhow::bail!("{} does not exist", file);
        }
    } else if !path.is_file() {
        anyhow::bail!("{} is not a file", file);
    }

    Ok(())
}
