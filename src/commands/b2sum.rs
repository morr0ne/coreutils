use std::{env::Args, fs, path::Path, process::ExitCode};

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
                Value(value) => files.push(value),
                Long("help") => {
                    println!("Usage: b2sum");
                    return Ok(ExitCode::SUCCESS);
                }
                _ => return Err(arg.unexpected().into()),
            }
        }

        files
    };

    for path in files {
        let file = fs::read(&path).expect("Failed to read file"); // TODO: Error handling

        let mut hasher = Blake2b512::new();

        hasher.update(file);

        let hash = hasher.finalize();

        println!(
            "{}  {}",
            hex::encode(hash),
            Path::new(&path).file_name().unwrap().to_str().unwrap()
        ); // TODO: Error handling
    }

    Ok(ExitCode::SUCCESS)
}
