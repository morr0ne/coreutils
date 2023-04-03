use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn vdir(args: Args) -> Result {
    let matches = new_command(
        "vdir",
        "List information about the FILEs (the current directory by default)",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
