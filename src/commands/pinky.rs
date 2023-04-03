use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn pinky(args: Args) -> Result {
    let matches = new_command(
        "pinky",
        "A lightweight 'finger' program;  print user information",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
