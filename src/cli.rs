use clap::{ArgGroup, Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "tx", version, about = "A project manager built on top of tmux")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(about = "List sessions, layouts, fragments. Enable listing directories with -a")]
    Ls {
        #[arg(short)]
        all: bool,
    },

    #[command(
        about = "Edit a layout, fragment, or the directories file",
        group(
            ArgGroup::new("target")
                .required(true)
                .args(&["layout", "fragment", "dirs"])
        )
    )]
    Edit(EditArgs),
}

#[derive(Args, Debug)]
pub struct EditArgs {
    #[arg(short = 'l', value_name = "LAYOUT")]
    pub layout: Option<String>,

    #[arg(short = 'f', value_name = "FRAGMENT")]
    pub fragment: Option<String>,

    #[arg(short = 'd', long = "dirs")]
    pub dirs: bool,
}
