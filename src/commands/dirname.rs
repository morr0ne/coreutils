use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn dirname(args: Args) -> Result {
    let matches = new_command(
        "dirname",
        "Output each NAME with its last non-slash component and trailing slashes
        removed; if NAME contains no /'s, output '.' (meaning the current directory)",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
