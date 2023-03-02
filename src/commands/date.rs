use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn date(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "date",
        "Display date and time in the given FORMAT",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
