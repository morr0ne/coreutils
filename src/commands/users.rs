use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn users(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "users",
        "Output who is currently logged in according to FILE",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
