use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn factor(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "factor",
        "Print the prime factors of each specified integer NUMBER.  If none
        are specified on the command line, read them from standard input",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
