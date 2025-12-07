use crate::managers::resources::manager_trait::ResourceManager;

const FRAGMENTS_DIR: &str = "fragments";
const FRAGMENT_TEMPLATE: &str = "fragment.template.toml";

pub struct FragmentsManager;

impl ResourceManager for FragmentsManager {
    fn get_resources_dir(&self) -> &'static str {
        FRAGMENTS_DIR
    }

    fn get_resource_template_file(&self) -> &'static str {
        FRAGMENT_TEMPLATE
    }
}
