use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn expand(args: Args) -> Result {
    let matches = new_command(
        "expand",
        "Convert tabs in each FILE to spaces, writing to standard output",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
