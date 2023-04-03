use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn unexpand(args: Args) -> Result {
    let matches = new_command(
        "unexpand",
        "Convert blanks in each FILE to tabs, writing to standard output",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
