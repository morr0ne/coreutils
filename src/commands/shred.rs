use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn shred(args: Args, multicall: bool) -> Result {
    let matches = new_command(
        "shred",
        "Overwrite the specified FILE(s) repeatedly, in order to make it harder for even very expensive hardware probing to recover the data",
        multicall,
    )
    .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
