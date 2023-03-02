use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn sort(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "sort",
        "Write sorted concatenation of all FILE(s) to standard output",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
