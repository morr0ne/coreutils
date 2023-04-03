use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn mktemp(args: Args) -> Result {
    let matches = new_command(
        "mktemp",
        "Create a temporary file or directory, safely, and print its name",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
