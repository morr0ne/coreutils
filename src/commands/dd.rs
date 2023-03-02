use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn dd(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "dd",
        "Copy a file, converting and formatting according to the operands",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
