use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn sha224sum(args: Args) -> Result {
    let matches = new_command(
        "sha224sum",
        "Print or check SHA224 (224-bit) checksums",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
