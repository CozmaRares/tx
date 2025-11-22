use anyhow::Result;

use super::{list, pick};

pub fn handle_switch() -> Result<()> {
    if let Some(first) = list::list(false)?.first() {
        pick::open_selection(&first.to_string(0))
    } else {
        Ok(())
    }
}
