use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn tsort(args: Args) -> Result {
    let matches = new_command(
        "tsort",
        "Write totally ordered list consistent with the partial ordering in FILE",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
