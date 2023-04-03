use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn stat(args: Args) -> Result {
    let matches = new_command(
        "stat",
        "Display file or file system status",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
