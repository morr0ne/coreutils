use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn ln(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "ln",
        "Create hard links by default, symbolic links with --symbolic",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
