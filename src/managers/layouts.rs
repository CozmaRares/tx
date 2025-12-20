use std::{collections::HashMap, fs};

use crate::managers::common::{TX_ROOT, ensure_dir_exists, ensure_file_exists};

const LAYOUTS_DIR: &str = "layouts";
const LAYOUT_TEMPLATE_FILE: &str = "layout.template.toml";
const FILE_EXT: &str = ".toml";
const TEMPLATE_LOCATION: &str = "templates";

pub struct LayoutsManager;

impl LayoutsManager {
    pub fn ensure_layouts_structure() -> anyhow::Result<()> {
        ensure_dir_exists(&format!("{}/{}", TX_ROOT.as_str(), LAYOUTS_DIR))?;
        ensure_file_exists(&LayoutsManager::get_template_path(), || {
            panic!(
                "Template file not found at path {}",
                LayoutsManager::get_template_path()
            )
        })?;
        Ok(())
    }

    pub fn create_file_path(name: &str) -> String {
        format!("{}/{}/{}{}", TX_ROOT.as_str(), LAYOUTS_DIR, name, FILE_EXT)
    }

    pub fn get_template_path() -> String {
        format!(
            "{}/{}/{}",
            TX_ROOT.as_str(),
            TEMPLATE_LOCATION,
            LAYOUT_TEMPLATE_FILE
        )
    }

    pub fn create_if_not_exists(name: &str) -> anyhow::Result<String> {
        let file_path = LayoutsManager::create_file_path(name);
        ensure_file_exists(&file_path, || create_from_template(name))?;
        Ok(file_path)
    }

    pub fn read(name: &str) -> anyhow::Result<String> {
        let file_path = LayoutsManager::create_file_path(name);
        let contents = fs::read_to_string(file_path)?;
        Ok(contents)
    }

    pub fn get_all() -> anyhow::Result<Vec<String>> {
        let dir_path = format!("{}/{}", TX_ROOT.as_str(), LAYOUTS_DIR);
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

#[derive(Debug)]
struct TemplateParams {
    name: String,
}

impl Into<HashMap<String, String>> for TemplateParams {
    fn into(self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("{{name}}".to_string(), self.name);
        map
    }
}

fn create_from_template(name: &str) -> String {
    let mut template = fs::read_to_string(LayoutsManager::get_template_path()).unwrap();
    let params = TemplateParams {
        name: name.to_string(),
    };

    let map: HashMap<String, String> = params.into();

    for (key, value) in map.iter() {
        template = template.replace(key, value);
    }

    template
}
