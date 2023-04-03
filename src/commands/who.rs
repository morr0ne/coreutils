use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn who(args: Args) -> Result {
    let matches = new_command(
        "who",
        "Print information about users who are currently logged in",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
