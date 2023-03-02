use std::{env::Args, ffi::OsString, fs, process::ExitCode};

use clap::{builder::ValueParser, Arg, ArgAction};
use data_encoding::BASE64;

use crate::{util::new_command, Result};

// TODO: Actually handle all the formats instead of defaulting to BASE64
// TODO: handle all args
// TODO: column wrap

pub fn basenc(args: Args, multicall: bool) -> Result {
    let matches = new_command("basenc", "basenc encode or decode FILE,", multicall)
        .arg(
            Arg::new("FILE")
                .index(1)
                .action(ArgAction::Append)
                .value_name("FILE")
                .value_hint(clap::ValueHint::FilePath)
                .value_parser(ValueParser::os_string()),
        )
        .arg(Arg::new("base64").long("base64").action(ArgAction::SetTrue))
        .arg(
            Arg::new("base64url")
                .long("base64url")
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new("base32").long("base32").action(ArgAction::SetTrue))
        .arg(
            Arg::new("base32hex")
                .long("base32hex")
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new("base16").long("base16").action(ArgAction::SetTrue))
        .get_matches_from(args);

    let file = matches
        .get_one::<OsString>("FILE")
        .expect("Failed to get arg");

    let file = fs::read(file).expect("Failed to read file");

    println!("{}", BASE64.encode(&file));

    Ok(ExitCode::SUCCESS)
}
