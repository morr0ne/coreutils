use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn logname(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "logname",
        "Print the user's login name",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
