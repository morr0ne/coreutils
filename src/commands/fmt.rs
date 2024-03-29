use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn fmt(args: Args) -> Result {
    let matches = new_command(
        "fmt",
        "Reformat each paragraph in the FILE(s), writing to standard output",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
