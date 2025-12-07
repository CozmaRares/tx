use crate::managers::resources::manager_trait::ResourceManager;

const LAYOUTS_DIR: &str = "layouts";
const LAYOUT_TEMPLATE: &str = "layout.template.toml";

pub struct LayoutsManager;

impl ResourceManager for LayoutsManager {
    fn get_resources_dir(&self) -> &'static str {
        LAYOUTS_DIR
    }

    fn get_resource_template_file(&self) -> &'static str {
        LAYOUT_TEMPLATE
    }
}
