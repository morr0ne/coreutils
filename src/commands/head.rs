use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn head(args: Args) -> Result {
    let matches = new_command(
        "head",
        "Print the first 10 lines of each FILE to standard output.",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
