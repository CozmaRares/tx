mod cli;
mod data;
mod fs;
mod runner;
mod tmux;

use std::{cmp::Ordering, collections::HashSet, path::Path};

use anyhow::{Ok, Result};
use cli::*;
use data::Data;

fn main() -> Result<()> {
    let command = cli::parse_args()?;

    match command {
        Command::Help => print_help(),
        Command::Ls { all } => list(all),
        Command::Preview { kind, value } => todo!(),
        Command::Edit(resource) => todo!(),
        Command::Pick => todo!(),
        Command::Switch => todo!(),
        Command::Sesh => todo!(),
        Command::NewSession(session_location) => todo!(),
        Command::Attach => todo!(),
    }
}

fn print_help() -> Result<()> {
    println!(
        "(￣∇￣)ゞ
Heya! Are you lost in the depths of the void? Here's a few tips to get you back on track:

    help                   --  Because sometimes you just don't know... ¯\\_(ツ)_/¯

    # Listing and Inspection
    ls                     --  Only the important shi- ...uhh, I mean active sessions and layouts, and fragments!
       -a                  --  Gimme all you have! (not blazingly fast)
    preview -s [session]   --  Peeks into the session's soul.
            -l [layout]    --  Whats in there? *taps layout*
            -f [fragment]  --  What are THOOSE?
            -d [dir]       --  Like window shopping for code.

    # Editing
    edit -l [layout]       --  Where and mistakes happen.
         -f [fragment]     --  Lego! Careful not to step on it!
         -d                --  Your chance to edit the library of Alexandria.

    # Session Management
    pick                   --  Limited stock. Session or layout?
    switch                 --  I changed my mind. Last session was better.
    sesh                   --  Which directory we're dropping, bois?
    [input]                --  New session. Who dis?
                           --    Matches layout? Instant architecture!
                           --    Is fragment? DIY!
                           --    Otherwise: Good ol' fashioned session (but it's your fault if it doesn't work).
    .                      --  Session! Where? Here!
    (no arguments)         --  Get back with your last ex(session)."
    );

    Ok(())
}

fn list(all: bool) -> Result<()> {
    let mut sessions = Data::get_sessions()?;
    let session_names: HashSet<_> = sessions.iter().map(|s| s.name.clone()).collect();
    let layouts: HashSet<_> = Data::get_layouts()?.into_iter().collect();
    let fragments = Data::get_fragments()?;
    let mut dirs = None;
    let mut dirs_last_2_parts = None;

    if all {
        let dirs2 = Data::get_dir_paths()?;
        dirs = Some(dirs2.clone());
        dirs_last_2_parts = Some(
            dirs2
                .iter()
                .filter_map(|dir| {
                    let path = Path::new(dir);
                    let mut components = path.components().rev();
                    let last = components.next()?.as_os_str().to_string_lossy();
                    let second_last = components.next()?.as_os_str().to_string_lossy();
                    Some(format!("{}/{}", second_last, last))
                })
                .collect::<Vec<_>>(),
        )
    }

    let mut max_len = session_names.iter().map(|s| s.len()).max().unwrap_or(0);
    max_len = max_len.max(layouts.iter().map(|s| s.len()).max().unwrap_or(0));
    max_len = max_len.max(fragments.iter().map(|s| s.len()).max().unwrap_or(0));

    if let Some(dirs_last_2_parts) = &dirs_last_2_parts {
        max_len = max_len.max(dirs_last_2_parts.iter().map(|s| s.len()).max().unwrap_or(0));
    }

    sessions.sort_by(|a, b| match a.is_attached.cmp(&b.is_attached) {
        Ordering::Equal => b.last_attached.cmp(&a.last_attached),
        other => other,
    });

    for session in sessions {
        println!(
            "{:<max_len$} (session) ({} windows){}",
            session.name,
            session.num_windows,
            if session.is_attached {
                " (attached)"
            } else {
                ""
            },
            max_len = max_len
        );
    }

    for layout in layouts.difference(&session_names) {
        println!("{:<max_len$} (layout)", layout, max_len = max_len);
    }

    for fragment in fragments {
        println!("{:<max_len$} (fragment)", fragment, max_len = max_len);
    }

    for dir in dirs_last_2_parts.unwrap_or_default() {
        println!("{:<max_len$} (directory)", dir, max_len = max_len);
    }

    Ok(())
}
