use std::env;

use anyhow::{bail, Result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Unknown arguments: {0:?}")]
    UnknownArguments(Vec<String>),

    #[error("Ls requires either -a or no arguments, but got: {0}")]
    UnknownLsFlag(String),

    #[error("Preview expects one of the following flags: -s, -l, -f, -d")]
    MissingPreviewFlag,

    #[error("Unknown preview flag: {0}")]
    UnknownPreviewFlag(String),

    #[error("Edit expects one of the following flags: -s, -l, -f, -d")]
    MissingEditFlag,

    #[error("Unknown edit flag: {0}")]
    UnknownEditFlag(String),

    #[error("Expected value after flag: {0}")]
    ExpectedValueAfterFlag(String),
}

#[derive(Debug)]
pub enum Command {
    Help,
    Ls { all: bool },
    Preview { kind: PreviewKind, value: String },
    Edit(Resource),
    Pick,
    Switch,
    Sesh,
    NewSession(SessionLocation),
    Attach,
}

#[derive(Debug)]
pub enum PreviewKind {
    Session,
    Layout,
    Fragment,
    Directory,
}

#[derive(Debug)]
pub enum Resource {
    Layout(String),
    Fragment(String),
    DirectoriesFile,
}

#[derive(Debug)]
pub enum SessionLocation {
    OpenHere,
    FromInput(String),
}

pub fn parse_args() -> Result<Command> {
    let mut args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Ok(Command::Attach);
    }

    let command = match args.remove(0).as_str() {
        "help" => Command::Help,
        "pick" => Command::Pick,
        "switch" => Command::Switch,
        "sesh" => Command::Sesh,
        "." => Command::NewSession(SessionLocation::OpenHere),
        "ls" => {
            if args.is_empty() {
                Command::Ls { all: false }
            } else if args.get(0) == Some(&"-a".to_string()) {
                args.remove(0);
                Command::Ls { all: true }
            } else {
                bail!(CliError::UnknownLsFlag(args.remove(0)))
            }
        }
        "preview" => {
            if args.is_empty() {
                bail!(CliError::MissingPreviewFlag);
            }

            let flag = args.remove(0);
            let kind = match flag.as_str() {
                "-s" => PreviewKind::Session,
                "-l" => PreviewKind::Layout,
                "-f" => PreviewKind::Fragment,
                "-d" => PreviewKind::Directory,
                _ => bail!(CliError::UnknownPreviewFlag(flag.to_string())),
            };

            if args.is_empty() {
                bail!(CliError::ExpectedValueAfterFlag(flag));
            }
            let value = args.remove(0);
            Command::Preview { kind, value }
        }
        "edit" => {
            if args.is_empty() {
                bail!(CliError::MissingEditFlag);
            }
            let flag = args.remove(0);

            let resource = if flag == "-d" {
                Resource::DirectoriesFile
            } else {
                let lambda = match flag.as_str() {
                    "-l" => |s| Resource::Layout(s),
                    "-f" => |s| Resource::Fragment(s),
                    _ => bail!(CliError::UnknownEditFlag(flag.to_string())),
                };

                if args.is_empty() {
                    bail!(CliError::ExpectedValueAfterFlag(flag));
                }

                lambda(args.remove(0))
            };

            Command::Edit(resource)
        }
        other => Command::NewSession(SessionLocation::FromInput(other.to_string())),
    };

    if !args.is_empty() {
        bail!(CliError::UnknownArguments(args));
    }

    Ok(command)
}
