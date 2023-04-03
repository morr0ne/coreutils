use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn kill(args: Args) -> Result {
    let matches = new_command(
        "kill",
        "Forcibly terminate a process",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
