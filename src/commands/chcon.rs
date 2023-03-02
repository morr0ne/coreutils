use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn chcon(args: Args, multicall: bool) -> Result {
    let matches = new_command("chcon", "Change the SELinux security context of each FILE to CONTEXT", multicall)
        .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
