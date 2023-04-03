use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn stdbuf(args: Args) -> Result {
    let matches = new_command(
        "stdbuf",
        "Run COMMAND, with modified buffering operations for its standard streams",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
