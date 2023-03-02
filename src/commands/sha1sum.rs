use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn sha1sum(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "sha1sum",
        "Print or check SHA1 (160-bit) checksums",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
