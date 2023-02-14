use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

pub fn r#true(args: Args, multicall: bool) -> Result {
    new_command(
        "true",
        "Exit with a status code indicating success.",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
