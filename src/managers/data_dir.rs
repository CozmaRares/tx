use crate::managers::{
    DirsManager, FragmentsManager, LayoutsManager, ResourceManager,
    common::{TX_DIR, ensure_dir_exists},
};

pub struct DataDirManager;

impl DataDirManager {
    pub fn ensure_tx_dir_structure() -> anyhow::Result<()> {
        ensure_dir_exists(TX_DIR)?;

        DirsManager::ensure_dirs_file()?;

        LayoutsManager.ensure_resource_dir_structure()?;
        FragmentsManager.ensure_resource_dir_structure()?;

        Ok(())
    }
}
