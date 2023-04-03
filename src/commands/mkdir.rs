use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn mkdir(args: Args) -> Result {
    let matches = new_command(
        "mkdir",
        "Create the DIRECTORY(ies), if they do not already exist",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
