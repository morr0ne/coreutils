use std::{env::Args, process::ExitCode};

use crate::{util::new_command, Result};

// TODO: Implement command

pub fn chgrp(args: Args) -> Result {
    let matches = new_command("chgrp", "Change the group of each FILE to GROUP")
        .get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
