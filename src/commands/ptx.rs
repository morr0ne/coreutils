use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn ptx(args: Args) -> Result {
    let matches = new_command(
        "ptx",
        "Output a permuted index, including context, of the words in the input files",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
