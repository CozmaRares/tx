use clap::{ArgGroup, Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "tx", version, about = "A project manager built on top of tmux")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
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
            ArgGroup::new("target")
                .required(true)
                .args(&["layout", "dirs"])
        )
    )]
    Edit(EditArgs),
}

#[derive(Args, Debug)]
pub struct EditArgs {
    #[arg(short = 'l', value_name = "LAYOUT")]
    pub layout: Option<String>,

    #[arg(short = 'd', long = "dirs")]
    pub dirs: bool,
}
