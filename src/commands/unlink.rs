use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn unlink(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "unlink",
        "Call the unlink function to remove the specified FILE",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
