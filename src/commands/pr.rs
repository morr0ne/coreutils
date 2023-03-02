use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn pr(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "pr",
        "Paginate or columnate FILE(s) for printing",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
