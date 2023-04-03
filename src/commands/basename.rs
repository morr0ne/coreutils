use std::{env::Args, process::ExitCode};

use clap::Arg;

use crate::{util::new_command, Result};

// TODO: handle all args
// TODO: Don't panic.
// TODO: Make program actually do what is supposed to. duh

pub fn basename(args: Args) -> Result {
    let matches = new_command(
        "basename",
        "Remove any leading directory components from NAME",
        
    )
    .arg(Arg::new("NAME").value_name("NAME"))
    .get_matches_from(args);

    let name = matches
        .get_one::<String>("NAME")
        .expect("Failed to get arg");

    println!("{name}");

    Ok(ExitCode::SUCCESS)
}
