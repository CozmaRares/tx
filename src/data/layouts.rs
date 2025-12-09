use crate::commands::bat;
use crate::managers::{LayoutsManager};

#[derive(Debug, Clone)]
pub struct TxLayout(pub String);

impl TxLayout {
    pub fn get_all() -> anyhow::Result<Vec<TxLayout>> {
        let files = LayoutsManager::get_all()?;
        Ok(files.into_iter().map(TxLayout).collect())
    }

    pub fn preview(&self) -> anyhow::Result<()> {
        bat::preview_file(LayoutsManager::create_file_path(&self.0))
    }

    pub fn open(name: &str) -> anyhow::Result<()> {
        todo!("Open layout")
    }
}
