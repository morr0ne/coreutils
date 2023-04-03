use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn touch(args: Args) -> Result {
    let matches = new_command(
        "touch",
        "Update the access and modification times of each FILE to the current time",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
