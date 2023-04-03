use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn join(args: Args) -> Result {
    let matches = new_command(
        "join",
        "For each pair of input lines with identical join fields, write a line to standard output",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
