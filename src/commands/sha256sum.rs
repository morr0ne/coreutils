use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn sha256sum(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "sha256sum",
        "Print or check SHA256 (256-bit) checksums",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
