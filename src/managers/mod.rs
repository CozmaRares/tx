mod common;
mod data_dir;
mod dirs;
mod resources;

pub use data_dir::DataDirManager;
pub use dirs::DirsManager;
pub use resources::{
    fragments::FragmentsManager, layouts::LayoutsManager, manager_trait::ResourceManager,
};
