use serde::Deserialize;

use crate::{
    commands::{bat, tmux::TmuxSessionBuilder},
    managers::LayoutsManager,
};

#[derive(Debug, Clone)]
pub struct TxLayout(pub String);

impl TxLayout {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn get_all() -> anyhow::Result<Vec<TxLayout>> {
        let files = LayoutsManager::get_all()?;
        Ok(files.into_iter().map(TxLayout).collect())
    }

    pub fn preview(&self) -> anyhow::Result<()> {
        bat::preview_file(LayoutsManager::create_file_path(&self.0))
    }

    pub fn open(self) -> anyhow::Result<()> {
        let layout_contents = LayoutsManager::read(&self.0)?;
        let config: TxLayoutConfig = toml::from_str(&layout_contents)?;
        config.open()
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
    name: String,
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
    pub fn open(self) -> anyhow::Result<()> {
        let mut builder = TmuxSessionBuilder::new(&self.project.name, self.project.root);

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
