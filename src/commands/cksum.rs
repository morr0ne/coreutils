use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn cksum(args: Args) -> Result {
    let matches = new_command(
        "cksum",
        "Print or verify checksums",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
