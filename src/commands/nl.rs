use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn nl(args: Args) -> Result {
    let matches = new_command(
        "nl",
        "Write each FILE to standard output, with line numbers added",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
