use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn pwd(args: Args) -> Result {
    let matches = new_command(
        "pwd",
        "Print the full filename of the current working directory",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
