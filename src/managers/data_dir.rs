use crate::managers::{
    DirsManager, LayoutsManager,
    common::{TX_DIR, ensure_dir_exists},
};

pub struct DataDirManager;

impl DataDirManager {
    pub fn ensure_tx_dir_structure() -> anyhow::Result<()> {
        ensure_dir_exists(TX_DIR)?;

        DirsManager::ensure_dirs_file()?;
        LayoutsManager::ensure_layouts_structure()?;

        Ok(())
    }
}
