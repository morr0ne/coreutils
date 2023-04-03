use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn tail(args: Args) -> Result {
    let matches = new_command(
        "tail",
        "Print the last 10 lines of each FILE to standard output",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
