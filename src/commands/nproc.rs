use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn nproc(args: Args) -> Result {
    let matches = new_command(
        "nproc",
        "Print the number of processing units available to the current process, which may be less than the number of online processors",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
