use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn sum(args: Args) -> Result {
    let matches = new_command(
        "sum",
        "Print or check BSD (16-bit) checksums",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
