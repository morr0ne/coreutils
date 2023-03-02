use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn truncate(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "truncate",
        "Shrink or extend the size of each FILE to the specified size",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
