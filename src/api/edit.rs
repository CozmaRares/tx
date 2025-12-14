use crate::{
    cli::{EditArgs, EditResource},
    commands::runner::execvp,
    managers::{DirsManager, LayoutsManager},
};

pub fn handle_edit(args: EditArgs) -> anyhow::Result<()> {
    let file_path;

    if let Some(layout_resource) = args.layout {
        file_path = match layout_resource {
            EditResource::Template => LayoutsManager::get_template_path(),
            EditResource::WithName(name) => LayoutsManager::create_if_not_exists(&name)?,
        }
    } else {
        file_path = DirsManager::get_dirs_file();
    }

    let editor = std::env::var("EDITOR").unwrap_or("vi".to_string());
    execvp(&[&editor, &file_path])
}
