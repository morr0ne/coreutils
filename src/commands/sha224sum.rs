use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn sha224sum(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "sha224sum",
        "Print or check SHA224 (224-bit) checksums",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
