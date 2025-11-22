use anyhow::Result;

use crate::{
    cli::EditResource,
    runner::{
        execvp,
        fs::{TxDirectory, TxFragment, TxLayout},
    },
};

pub fn handle_edit(resource: EditResource) -> Result<()> {
    let file = match resource {
        EditResource::Layout(name) => TxLayout::get_file(name)?,
        EditResource::Fragment(name) => TxFragment::get_file(name)?,
        EditResource::DirectoriesFile => TxDirectory::get_dirs_file(),
    };

    let editor = std::env::var("EDITOR").unwrap_or("vi".to_string());

    execvp(&[&editor, &file])
}
