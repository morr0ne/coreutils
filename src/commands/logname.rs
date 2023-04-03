use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn logname(args: Args) -> Result {
    let matches = new_command(
        "logname",
        "Print the user's login name",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
