use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn rmdir(args: Args) -> Result {
    let matches = new_command(
        "rmdir",
        "Remove the DIRECTORY(ies), if they are empty",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
