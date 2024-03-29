use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn link(args: Args) -> Result {
    let matches = new_command(
        "link",
        "Call the link function to create a link named FILE2 to an existing FILE1.",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
