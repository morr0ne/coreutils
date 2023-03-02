use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn printenv(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "printenv",
        "Print the values of the specified environment VARIABLE(s).
        If no VARIABLE is specified, print name and value pairs for them all",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
