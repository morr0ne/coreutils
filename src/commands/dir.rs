use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn dir(args: Args) -> Result {
    let matches = new_command(
        "dir",
        "List information about the FILEs (the current directory by default).
        Sort entries alphabetically if none of -cftuvSUX nor --sort is specified",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
