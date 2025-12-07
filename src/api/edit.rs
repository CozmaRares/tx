use crate::{
    cli::EditArgs,
    commands::runner::execvp,
    managers::{DirsManager, FragmentsManager, LayoutsManager, ResourceManager},
};

pub fn handle_edit(args: EditArgs) -> anyhow::Result<()> {
    let EditArgs {
        layout, fragment, ..
    } = args;

    let file_path;

    if let Some(layout) = layout {
        file_path = LayoutsManager.create_resource_if_not_exists(&layout)?;
    } else if let Some(fragment) = fragment {
        file_path = FragmentsManager.create_resource_if_not_exists(&fragment)?;
    } else {
        file_path = DirsManager::get_dirs_file();
    }

    let editor = std::env::var("EDITOR").unwrap_or("vi".to_string());

    execvp(&[&editor, &file_path])
}
