use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn sha512sum(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "sha512sum",
        "Print or check SHA512 (512-bit) checksums",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
