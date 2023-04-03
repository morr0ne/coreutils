use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn od(args: Args) -> Result {
    let matches = new_command(
        "od",
        "Write an unambiguous representation, octal bytes by default,
        of FILE to standard output.  With more than one FILE argument,
        concatenate them in the listed order to form the input",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
