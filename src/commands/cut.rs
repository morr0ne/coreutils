use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn cut(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "cut",
        "Print selected parts of lines from each FILE to standard output",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
