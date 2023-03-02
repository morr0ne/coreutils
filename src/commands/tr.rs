use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn tr(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "tr",
        "Translate, squeeze, and/or delete characters from standard input",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
