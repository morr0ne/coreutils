use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn chroot(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "chroot",
        "Run COMMAND with root directory set to NEWROOT",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
