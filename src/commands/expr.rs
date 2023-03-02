use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn expr(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "expr",
        "Print the value of EXPRESSION to standard output",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
