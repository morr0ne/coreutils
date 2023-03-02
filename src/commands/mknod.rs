use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn mknod(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "mknod",
        "Create the special file NAME of the given TYPE",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
