use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn numfmt(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "numfmt",
        "Reformat NUMBER(s), or the numbers from standard input if none are specified",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
