use crate::{
    api::ls::{self, ListFilter, LsData},
    data::TmuxSession,
};

pub fn handle_switch() -> anyhow::Result<()> {
    let sessions = ls::list(ListFilter::JustSessions)?;
    let last_session = sessions.first();

    if let Some(LsData::Session(session)) = last_session {
        TmuxSession::open(&session.name)?;
    }

    Ok(())
}
