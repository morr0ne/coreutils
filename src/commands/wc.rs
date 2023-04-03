use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn wc(args: Args) -> Result {
    let matches = new_command(
        "wc",
        "Print newline, word, and byte counts for each FILE, and a total line if more than one FILE is specified.  A word is a non-zero-length sequence of printable characters delimited by white space.",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
