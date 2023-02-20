use std::{env::Args, ffi::OsString, fs, process::ExitCode};

use blake2::{Blake2b512, Digest};
use clap::{builder::ValueParser, Arg, ArgAction};

use crate::{util::new_command, Result};

pub fn uptime(args: Args, multicall: bool) -> Result {
    let matches =
        new_command("uptime", "Prints the system uptime", multicall).get_matches_from(args);

    Ok(ExitCode::SUCCESS)
}
