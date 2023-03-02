use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn install(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "install",
        "This install program copies files into destination",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
