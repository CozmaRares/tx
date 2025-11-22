use anyhow::Result;

pub fn handle_help() -> Result<()> {
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
