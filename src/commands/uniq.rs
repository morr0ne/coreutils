use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn uniq(args: Args) -> Result {
    let matches = new_command(
        "uniq",
        "Filter adjacent matching lines from INPUT (or standard input),writing to OUTPUT (or standard output)",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
