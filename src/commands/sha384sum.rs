use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn sha384sum(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "sha384sum",
        "Print or check SHA384 (384-bit) checksums",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
