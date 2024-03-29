use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn hostname(args: Args) -> Result {
    let matches = new_command(
        "hostname",
        "Display or set the system hostname",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
