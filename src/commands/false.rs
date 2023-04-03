use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn r#false(args: Args) -> Result {
    let matches = new_command(
        "false",
        "Exit with a status code indicating failure",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
