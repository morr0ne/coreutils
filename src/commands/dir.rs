use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn dir(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "dir",
        "List information about the FILEs (the current directory by default).
        Sort entries alphabetically if none of -cftuvSUX nor --sort is specified",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
