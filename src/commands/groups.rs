use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn groups(args: Args) -> Result {
    let matches = new_command(
        "groups",
        "Print group memberships for each USERNAME or the current process",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
