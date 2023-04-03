use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn csplit(args: Args) -> Result {
    let matches = new_command(
        "csplit",
        "Output pieces of FILE separated by PATTERN(s) to files 'xx00', 'xx01', ...,
        and output byte counts of each piece to standard output.",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
