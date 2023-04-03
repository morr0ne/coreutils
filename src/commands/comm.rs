use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn comm(args: Args) -> Result {
    let matches = new_command(
        "comm",
        "Compare sorted files FILE1 and FILE2 line by line.",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
