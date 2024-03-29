use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn chown(args: Args) -> Result {
    let matches = new_command(
        "chown",
        "Change the owner and/or group of each FILE to OWNER and/or GROUP",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
