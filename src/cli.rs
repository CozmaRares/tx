use clap::{ArgGroup, Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "tx", version, about = "A project manager built on top of tmux")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    pub session_name: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(about = "List sessions and layouts. Enable listing directories with -a")]
    Ls {
        #[arg(short)]
        all: bool,
    },

    #[command(
        about = "Edit a layout or the directories file",
        group(
            ArgGroup::new("edit args")
                .required(true)
                .args(&["layout", "dirs"])
        )
    )]
    Edit(EditArgs),

    #[command(about = "Remove a layout")]
    Rml { name: String },

    #[command(
        about = "Preview a session, layout or directory",
        group(
            ArgGroup::new("preview args")
                .required(true)
                .args(&["session", "layout", "directory"])
        )
    )]
    Preview(PreviewArgs),

    #[command(about = "Pick a running session or create one from a layout")]
    Pick,

    #[command(about = "Switch to the last session")]
    Switch,

    #[command(about = "Create a new session from the list of directories")]
    Sesh,

    #[command(about = "Create new session here", name = ".")]
    Dot,
}

#[derive(Args, Debug)]
pub struct EditArgs {
    #[arg(short = 'l', value_name = "LAYOUT")]
    pub layout: Option<EditResource>,

    #[arg(short = 'd', long = "dirs")]
    pub dirs: bool,
}

#[derive(Debug, Clone)]
pub enum EditResource {
    Template,
    WithName(String),
}

impl std::str::FromStr for EditResource {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "template" => Ok(EditResource::Template),
            _ => Ok(EditResource::WithName(s.to_string())),
        }
    }
}

#[derive(Args, Debug)]
pub struct PreviewArgs {
    #[arg(short = 's', value_name = "SESSION")]
    pub session: Option<String>,

    #[arg(short = 'l', value_name = "LAYOUT")]
    pub layout: Option<String>,

    #[arg(short = 'd', value_name = "DIRECTORY")]
    pub directory: Option<String>,
}
