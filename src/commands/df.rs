use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn df(args: Args) -> Result {
    let matches = new_command(
        "df",
        "Show information about the file system on which each FILE resides,
        or all file systems by default",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
