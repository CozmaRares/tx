use crate::commands::bat;
use crate::managers::{FragmentsManager, ResourceManager};

#[derive(Debug, Clone)]
pub struct TxFragment(pub String);

impl TxFragment {
    pub fn get_all() -> anyhow::Result<Vec<TxFragment>> {
        let files = FragmentsManager.list_resources()?;
        Ok(files.into_iter().map(TxFragment).collect())
    }

    pub fn preview(&self) -> anyhow::Result<()> {
        bat::preview_file(FragmentsManager.create_file_path(&self.0))
    }

    pub fn open(name: &str) -> anyhow::Result<()> {
        todo!("Open layout")
    }
}
