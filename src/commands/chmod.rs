use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn chmod(args: Args, multicall: bool) -> Result {
    let matches = new_command("chmod", "Change the mode of each FILE to MODE", multicall)
        .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
