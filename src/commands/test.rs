use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command
// TODO: Never used this command before, no clue what it does or why it doesn't print an help thingy

pub fn test(args: Args) -> Result {
    let matches = new_command(
        "test",
        "Does something??",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
