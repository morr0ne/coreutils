use std::{env::Args, ffi::OsString, fs, process::ExitCode};

use blake2::{Blake2b512, Digest};
use clap::{builder::ValueParser, Arg, ArgAction};

use crate::{util::new_command, Result};


// TODO: Actually parse all the args.

pub fn b2sum(args: Args, multicall: bool) -> Result {
    let matches = new_command("b2sum", "Compute or check BLAKE2b checksums", multicall)
        .arg(
            Arg::new("binary")
                .short('b')
                .long("binary")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("check")
                .short('c')
                .long("check")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("FILE")
                .index(1)
                .action(ArgAction::Append)
                .value_name("FILE")
                .value_hint(clap::ValueHint::FilePath)
                .value_parser(ValueParser::os_string()),
        )
        .get_matches_from(args);

    let file = matches
        .get_one::<OsString>("FILE")
        .expect("Failed to get arg");

    let file = fs::read(file).expect("Failed to read file");

    let mut hasher = Blake2b512::new();

    hasher.update(file);

    let hash = hasher.finalize();

    println!("{}", hex::encode(hash));

    Ok(ExitCode::SUCCESS)
}
