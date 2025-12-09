use std::fs;

use crate::managers::LayoutsManager;

pub fn handle_rml(name: String) -> anyhow::Result<()> {
    let file = LayoutsManager::create_file_path(&name);
    fs::remove_file(file)?;
    Ok(())
}
