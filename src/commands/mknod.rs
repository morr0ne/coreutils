use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn mknod(args: Args) -> Result {
    let matches = new_command(
        "mknod",
        "Create the special file NAME of the given TYPE",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
