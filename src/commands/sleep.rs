use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn sleep(args: Args) -> Result {
    let matches = new_command(
        "sleep",
        "Pause for NUMBER seconds",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
