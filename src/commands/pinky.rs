use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn pinky(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "pinky",
        "A lightweight 'finger' program;  print user information",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
