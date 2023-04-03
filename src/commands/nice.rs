use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn nice(args: Args) -> Result {
    let matches = new_command(
        "nice",
        "Run COMMAND with an adjusted niceness, which affects process scheduling",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
