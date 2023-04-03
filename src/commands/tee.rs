use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn tee(args: Args) -> Result {
    let matches = new_command(
        "tee",
        "Copy standard input to each FILE, and also to standard output",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
