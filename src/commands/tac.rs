use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn tac(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "tac",
        "Write each FILE to standard output, last line first",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
