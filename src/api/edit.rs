use crate::{
    cli::EditArgs,
    commands::runner::execvp,
    managers::{DirsManager, LayoutsManager},
};

pub fn handle_edit(args: EditArgs) -> anyhow::Result<()> {
    let file_path;

    if let Some(layout) = args.layout {
        file_path = LayoutsManager::create_resource_if_not_exists(&layout)?;
    } else {
        file_path = DirsManager::get_dirs_file();
    }

    let editor = std::env::var("EDITOR").unwrap_or("vi".to_string());
    execvp(&[&editor, &file_path])
}
