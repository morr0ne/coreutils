use std::{
    env::Args,
    ffi::{OsStr, OsString},
    fs,
    io::{stdin, stdout, Read, Write},
    os::unix::ffi::OsStrExt,
    path::Path,
    process::ExitCode,
};

use blake2::{Blake2b512, Digest};
use lexopt::prelude::*;

use crate::Result;

// TODO: Actually parse all the args.
// TODO: Don't panic
// TODO: Had context to errors.

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
            let stdin = stdin().lock().read_to_end(&mut buf)?;
            buf
        } else {
            fs::read(&path)?
        };

        let mut hasher = Blake2b512::new();

        hasher.update(file);

        let hash = hasher.finalize();

        let mut stdout = stdout().lock();
        stdout.write_all(hex::encode(hash).as_bytes())?;
        stdout.write_all(b"  ")?;
        stdout.write_all(
            Path::new(&path)
                .file_name()
                .unwrap_or(OsStr::new("unknown"))
                .as_bytes(),
        )?;
        stdout.write_all(b"\n")?;
    }

    Ok(ExitCode::SUCCESS)
}
