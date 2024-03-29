use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn cat(args: Args) -> Result {
    let matches = new_command("cat", "Concatenate FILE(s) to standard output")
        .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
