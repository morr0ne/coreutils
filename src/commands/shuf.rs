use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn shuf(args: Args) -> Result {
    let matches = new_command(
        "shuf",
        "Write a random permutation of the input lines to standard output",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
