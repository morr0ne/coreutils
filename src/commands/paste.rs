use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn paste(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "paste",
        "Write lines consisting of the sequentially corresponding lines from each FILE, separated by TABs, to standard output",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
