use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn hostid(args: Args) -> Result {
    let matches = new_command(
        "hostid",
        "Print the numeric identifier (in hexadecimal) for the current host",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
