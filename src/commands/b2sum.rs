use std::{env::Args, ffi::OsString, fs, path::PathBuf, process::ExitCode};

use blake2::{Blake2b512, Digest};
use lexopt::prelude::*;

use crate::Result;

// TODO: Actually parse all the args.
// TODO: Don't panic

pub fn b2sum(args: Args) -> Result {
    let files = {
        let mut files = Vec::new();
        let mut parser = lexopt::Parser::from_args(args);

        while let Some(arg) = parser.next()? {
            match arg {
                Value(value) => files.push(PathBuf::from(value)),
                Long("help") => {
                    println!("Usage: b2sum");
                    return Ok(ExitCode::SUCCESS);
                }
                _ => return Err(arg.unexpected().into()),
            }
        }

        files
    };

    let file = fs::read(&files[0]).expect("Failed to read file");

    let mut hasher = Blake2b512::new();

    hasher.update(file);

    let hash = hasher.finalize();

    println!("{}", hex::encode(hash));

    Ok(ExitCode::SUCCESS)
}
