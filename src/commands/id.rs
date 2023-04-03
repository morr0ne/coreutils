use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn id(args: Args) -> Result {
    let matches = new_command(
        "id",
        "Print user and group information for each specified USER",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
