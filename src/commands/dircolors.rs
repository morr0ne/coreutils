use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn dircolors(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "dircolors",
        "Output commands to set the LS_COLORS environment variable",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
