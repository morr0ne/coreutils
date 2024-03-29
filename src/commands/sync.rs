use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn sync(args: Args) -> Result {
    let matches = new_command(
        "sync",
        "Synchronize cached writes to persistent storage",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
