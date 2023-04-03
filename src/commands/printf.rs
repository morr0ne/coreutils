use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn printf(args: Args) -> Result {
    let matches = new_command(
        "printf",
        "Print ARGUMENT(s) according to FORMAT, or execute according to OPTION:",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
