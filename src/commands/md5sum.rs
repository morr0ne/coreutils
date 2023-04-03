use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn md5sum(args: Args) -> Result {
    let matches = new_command(
        "md5sum",
        "Print or check MD5 (128-bit) checksums",
        
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
