use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn echo(args: Args) -> Result {
    let matches = new_command(
        "echo",
        "Echo the STRING(s) to standard output",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
