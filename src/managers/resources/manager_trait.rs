use std::{fs, path::Path};

use crate::managers::common::{TX_DIR, ensure_dir_exists, ensure_file_exists};
use anyhow::Context;

const RESOURCE_EXT: &str = ".toml";

pub trait ResourceManager {
    fn get_resources_dir(&self) -> &'static str;
    fn get_resource_template_file(&self) -> &'static str;

    fn ensure_resource_dir_structure(&self) -> anyhow::Result<()> {
        let dir = self.get_resources_dir();
        let template_file = self.get_resource_template_file();

        ensure_dir_exists(&format!("{}/{}", TX_DIR, dir))?;
        ensure_file_exists(
            &format!("{}/{}", TX_DIR, template_file),
            Option::<Box<dyn FnOnce() -> String>>::None,
        )
        .context(format!(
            "Resource template file is missing and cannot be created: {}",
            template_file
        ))?;
        Ok(())
    }

    fn create_file_path(&self, name: &str) -> String {
        format!(
            "{}/{}/{}{}",
            TX_DIR,
            self.get_resources_dir(),
            name,
            RESOURCE_EXT
        )
    }

    fn read_template(&self) -> String {
        let template_file = self.get_resource_template_file();
        fs::read_to_string(format!("{}/{}", TX_DIR, template_file)).unwrap()
    }

    fn create_resource_if_not_exists(&self, name: &str) -> anyhow::Result<String> {
        let file_path = self.create_file_path(name);
        ensure_file_exists(&file_path, Some(|| self.create_from_template(name)))?;
        Ok(file_path)
    }

    fn read_resource(&self, name: &str) -> anyhow::Result<String> {
        let file_path = self.create_file_path(name);
        let contents = fs::read_to_string(file_path)?;
        Ok(contents)
    }

    fn list_resources(&self) -> anyhow::Result<Vec<String>> {
        let dir = self.get_resources_dir();
        let dir_path = format!("{}/{}", TX_DIR, dir);
        let read_dir = fs::read_dir(dir_path)?;

        let files: Vec<_> = read_dir
            .into_iter()
            .filter_map(|result| result.ok())
            .filter(|entry| entry.path().is_file())
            .map(|entry| entry.file_name().into_string().unwrap())
            .filter(|name| name.ends_with(RESOURCE_EXT))
            .map(|name| name.strip_suffix(RESOURCE_EXT).unwrap().to_string())
            .collect();

        Ok(files)
    }

    fn create_from_template(&self, name: &str) -> String {
        let template = self.read_template();
        let params = TemplateParams {
            name: name.to_string(),
        };
        populate_template(template, params).unwrap()
    }
}

#[derive(Debug)]
struct TemplateParams {
    name: String,
}

fn populate_template(contents: String, params: TemplateParams) -> anyhow::Result<String> {
    let mut tera = tera::Tera::default();
    let mut context = tera::Context::new();
    context.insert("name", &params.name);
    Ok(tera.render_str(&contents, &context)?)
}
