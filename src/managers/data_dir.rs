use crate::managers::{
    DirsManager, LayoutsManager,
    common::{TX_ROOT, ensure_dir_exists},
};

pub struct DataDirManager;

impl DataDirManager {
    pub fn ensure_tx_dir_structure() -> anyhow::Result<()> {
        ensure_dir_exists(&TX_ROOT)?;

        DirsManager::ensure_dirs_file()?;
        LayoutsManager::ensure_layouts_structure()?;

        Ok(())
    }
}
