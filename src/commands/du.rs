use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn du(args: Args) -> Result {
    let matches = new_command(
        "du",
        "Summarize device usage of the set of FILEs, recursively for directories",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
