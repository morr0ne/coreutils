use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn mkfifo(args: Args) -> Result {
    let matches = new_command(
        "mkfifo",
        "Create named pipes (FIFOs) with the given NAMEs",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
