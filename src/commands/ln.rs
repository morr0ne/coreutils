use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn ln(args: Args) -> Result {
    let matches = new_command(
        "ln",
        "Create hard links by default, symbolic links with --symbolic",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
