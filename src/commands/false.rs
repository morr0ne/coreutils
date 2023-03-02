use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn r#false(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "false",
        "Exit with a status code indicating failure",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
