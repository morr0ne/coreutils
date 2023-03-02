use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn seq(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "seq",
        "Print numbers from FIRST to LAST, in steps of INCREMENT",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
