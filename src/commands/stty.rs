use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn stty(args: Args) -> Result {
    let matches = new_command(
        "stty",
        "Print or change terminal characteristics",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
