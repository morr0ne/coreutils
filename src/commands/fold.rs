use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn fold(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "fold",
        "Wrap input lines in each FILE, writing to standard output",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
