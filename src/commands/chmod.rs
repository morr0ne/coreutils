use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn chmod(args: Args) -> Result {
    let matches = new_command("chmod", "Change the mode of each FILE to MODE")
        .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
