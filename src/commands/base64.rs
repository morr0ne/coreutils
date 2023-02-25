use std::{env::Args, ffi::OsString, fs, process::ExitCode};

use clap::{builder::ValueParser, Arg, ArgAction};
use data_encoding::BASE64;

use crate::{util::new_command, Result};

// TODO: handle all args
// TODO: column wrap

pub fn base64(args: Args, multicall: bool) -> Result {
    let matches = new_command("base64", "Base64 encode or decode FILE,", multicall)
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

    println!("{}", BASE64.encode(&file));

    Ok(ExitCode::SUCCESS)
}
