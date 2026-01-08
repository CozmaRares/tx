use crate::{
    api::ls::{self, ListFilter},
    commands::tmux,
};

pub fn handle_new_session(name: String) -> anyhow::Result<()> {
    let ls_data = ls::list(ListFilter::Regular)?;

    for data in ls_data {
        match data {
            ls::LsData::Session(session) => {
                if session.name == name {
                    tmux::open_session(&session.name)?;
                }
            }
            ls::LsData::Layout(layout) => {
                if layout.name == name {
                    layout.open()?;
                }
            }

            _ => unreachable!(),
        }
    }

    Ok(())
}
