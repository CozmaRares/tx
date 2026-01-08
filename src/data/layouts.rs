use serde::Deserialize;

use crate::{
    commands::{bat, tmux::TmuxSessionBuilder},
    managers::LayoutsManager,
};

#[derive(Debug)]
pub struct TxLayout {
    pub name: String,
    overridden_root: Option<String>,
}

impl TxLayout {
    pub fn new(name: String) -> Self {
        Self {
            name,
            overridden_root: None,
        }
    }

    pub fn override_root(&mut self, root: String) {
        self.overridden_root = Some(root);
    }

    pub fn get_all() -> anyhow::Result<Vec<TxLayout>> {
        let files = LayoutsManager::get_all()?;
        Ok(files.into_iter().map(TxLayout::new).collect())
    }

    pub fn preview(&self) -> anyhow::Result<()> {
        bat::preview_file(LayoutsManager::create_file_path(&self.name))
    }

    pub fn open(self) -> anyhow::Result<()> {
        let layout_contents = LayoutsManager::read(&self.name)?;
        let mut config: TxLayoutConfig = toml::from_str(&layout_contents)?;

        match (&self.overridden_root, &config.project.root) {
            (Some(_), Some(_)) => {
                anyhow::bail!(
                    "You cannot override the root of layout {} because it already has a root",
                    self.name
                )
            }
            (Some(_), None) => config.project.root = self.overridden_root,
            _ => {}
        }

        config.open(&self.name)
    }

    pub fn find(layout: &str) -> Option<TxLayout> {
        let Ok(layouts) = TxLayout::get_all() else {
            return None;
        };
        layouts.into_iter().find(|l| l.name == layout)
    }
}

#[derive(Debug, Deserialize)]
struct TxLayoutConfig {
    project: ProjectConfig,
    #[serde(rename = "window", default)]
    windows: Vec<Window>,
}

#[derive(Debug, Deserialize)]
struct ProjectConfig {
    name: Option<String>,
    root: Option<String>,
    default_window: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Window {
    name: String,
    #[serde(rename = "pane", default)]
    panes: Vec<Pane>,
    default_pane: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct Pane {
    commands: Vec<String>,
    direction: Option<PaneDirection>,
    size: Option<usize>,
}

impl TxLayoutConfig {
    pub fn open(self, layout_name: &str) -> anyhow::Result<()> {
        let mut builder = match (self.project.name, self.project.root) {
            (Some(name), Some(root)) => TmuxSessionBuilder::new_named(name, root),
            (None, Some(root)) => TmuxSessionBuilder::new_from_dir(root),
            (None, None) => TmuxSessionBuilder::default(),
            (Some(_), None) => {
                anyhow::bail!(
                    "Layout {} has a name set, but no root. Root is required if name is set",
                    layout_name
                )
            }
        };

        builder.create_session()?;

        for (i, window) in self.windows.iter().enumerate() {
            if i == 0 {
                builder.rename_window(&window.name)?;
            } else {
                builder.create_window(&window.name)?;
            }

            for (i, pane) in window.panes.iter().enumerate() {
                if i == 0 && (pane.direction.is_some() || pane.size.is_some()) {
                    anyhow::bail!("First pane cannot have a split direction or size");
                } else if i > 0 && (pane.direction.is_none() || pane.size.is_none()) {
                    anyhow::bail!("All panes after the first must have a split direction and size");
                }

                if let Some(direction) = &pane.direction {
                    let size = pane.size.unwrap();
                    builder.split_pane(direction.into(), size)?;
                }

                for command in &pane.commands {
                    builder.run_command(command)?;
                }
            }

            if let Some(default_pane) = window.default_pane {
                builder.select_pane(default_pane)?;
            }
        }

        if let Some(default_window) = self.project.default_window {
            builder.select_window(&default_window)?;
        }

        builder.open_session()
    }
}

#[derive(Debug, Deserialize)]
pub enum PaneDirection {
    #[serde(rename = "horizontal")]
    Horizontal,
    #[serde(rename = "vertical")]
    Vertical,
}

impl Into<&'static str> for &PaneDirection {
    fn into(self) -> &'static str {
        match self {
            PaneDirection::Horizontal => "-h",
            PaneDirection::Vertical => "-v",
        }
    }
}
