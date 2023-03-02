use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn pathchk(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "pathchk",
        "Diagnose invalid or unportable file names",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
