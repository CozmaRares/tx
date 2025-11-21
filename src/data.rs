use std::collections::HashSet;

use anyhow::Result;

use crate::{
    fs::{get_dir_paths, get_fragments, get_layouts},
    tmux::{get_tmux_sessions, TmuxSession},
};

pub struct Data;

impl Data {
    pub fn get_sessions() -> Result<Vec<TmuxSession>> {
        get_tmux_sessions()
    }

    pub fn get_layouts() -> Result<Vec<String>> {
        get_layouts()
    }

    pub fn get_fragments() -> Result<Vec<String>> {
        get_fragments()
    }

    pub fn get_dir_paths() -> Result<Vec<String>> {
        get_dir_paths()
    }
}
