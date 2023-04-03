use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

pub fn r#true(args: Args) -> Result {
    new_command(
        "true",
        "Exit with a status code indicating success.",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
