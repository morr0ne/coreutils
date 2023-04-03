use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn realpath(args: Args) -> Result {
    let matches = new_command(
        "realpath",
        "Print the resolved absolute file name",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
