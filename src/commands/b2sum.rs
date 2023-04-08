use std::{
    env::Args,
    ffi::{OsStr, OsString},
    fs,
    io::{stdin, Read},
    path::Path,
    process::ExitCode,
};

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

        if files.is_empty() {
            files.push(OsString::from("-"))
        }

        files
    };

    for path in files {
        let file = if path == OsStr::new("-") {
            let mut buf = Vec::new();
            let stdin = stdin().lock().read_to_end(&mut buf).unwrap();
            buf
        } else {
            // TODO: Error handling
            fs::read(&path).expect("Failed to read file")
        };

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
