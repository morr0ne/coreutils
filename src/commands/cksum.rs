use std::{env::Args, process::ExitCode};

use lexopt::prelude::*;

use crate::Result;

// TODO: Implement command

pub fn cksum(args: Args) -> Result {
    let mut parser = lexopt::Parser::from_args(args);

    while let Some(arg) = parser.next()? {
        if let Long("help") = arg {
            println!("Usage: cksum\nPrint or verify checksums");
            return Ok(ExitCode::SUCCESS);
        } else {
            return Err(arg.unexpected().into());
        }
    }

    Ok(ExitCode::SUCCESS)
}
