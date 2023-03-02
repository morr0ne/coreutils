use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn split(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "split",
        "Output pieces of FILE to PREFIXaa, PREFIXab, ...;",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
