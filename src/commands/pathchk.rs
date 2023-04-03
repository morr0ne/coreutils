use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn pathchk(args: Args) -> Result {
    let matches = new_command(
        "pathchk",
        "Diagnose invalid or unportable file names",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
