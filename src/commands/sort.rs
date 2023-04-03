use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn sort(args: Args) -> Result {
    let matches = new_command(
        "sort",
        "Write sorted concatenation of all FILE(s) to standard output",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
