use std::fs;

use crate::managers::common::{TX_DIR, ensure_dir_exists, ensure_file_exists};

const LAYOUTS_DIR: &str = "layouts";
const LAYOUT_TEMPLATE_FILE: &str = "layout.template.toml";
const FILE_EXT: &str = ".toml";
const DEFAULT_TEMPLATE_LOCATION: &str = "templates";

pub struct LayoutsManager;

impl LayoutsManager {
    pub fn ensure_layouts_structure() -> anyhow::Result<()> {
        ensure_dir_exists(&format!("{}/{}", TX_DIR, LAYOUTS_DIR))?;
        ensure_file_exists(
            &format!("{}/{}", TX_DIR, LAYOUT_TEMPLATE_FILE),
            create_template,
        )?;
        Ok(())
    }

    pub fn create_file_path(name: &str) -> String {
        format!("{}/{}/{}{}", TX_DIR, LAYOUTS_DIR, name, FILE_EXT)
    }

    pub fn create_resource_if_not_exists(name: &str) -> anyhow::Result<String> {
        let file_path = LayoutsManager::create_file_path(name);
        ensure_file_exists(&file_path, || create_from_template(name))?;
        Ok(file_path)
    }

    pub fn read_resource(name: &str) -> anyhow::Result<String> {
        let file_path = LayoutsManager::create_file_path(name);
        let contents = fs::read_to_string(file_path)?;
        Ok(contents)
    }

    pub fn get_all() -> anyhow::Result<Vec<String>> {
        let dir_path = format!("{}/{}", TX_DIR, LAYOUTS_DIR);
        let read_dir = fs::read_dir(dir_path)?;

        let files: Vec<_> = read_dir
            .into_iter()
            .filter_map(|result| result.ok())
            .filter(|entry| entry.path().is_file())
            .map(|entry| entry.file_name().into_string().unwrap())
            .filter(|name| name.ends_with(FILE_EXT))
            .map(|name| name.strip_suffix(FILE_EXT).unwrap().to_string())
            .collect();

        Ok(files)
    }
}

fn read_template() -> String {
    fs::read_to_string(format!("{}/{}", TX_DIR, LAYOUT_TEMPLATE_FILE)).unwrap()
}

fn create_template() -> String {
    let template_path = format!("{}/{}", DEFAULT_TEMPLATE_LOCATION, LAYOUT_TEMPLATE_FILE);
    fs::read_to_string(template_path).unwrap()
}

#[derive(Debug)]
struct TemplateParams {
    name: String,
}

fn create_from_template(name: &str) -> String {
    let template = read_template();
    let params = TemplateParams {
        name: name.to_string(),
    };

    let mut tera = tera::Tera::default();
    let mut context = tera::Context::new();
    context.insert("name", &params.name);

    tera.render_str(&template, &context).unwrap()
}
