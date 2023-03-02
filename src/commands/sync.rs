use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn sync(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "sync",
        "Synchronize cached writes to persistent storage",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
