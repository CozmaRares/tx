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
    pub layout: Option<String>,

    #[arg(short = 'd', long = "dirs")]
    pub dirs: bool,
}
