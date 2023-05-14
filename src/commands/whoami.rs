use std::{
    env::Args,
    io::{stdout, Write},
    process::ExitCode,
};

use lexopt::prelude::*;
use rustix::process::geteuid;

use crate::{utils::passwd::Parser, Result};

pub fn whoami(args: Args) -> Result {
    let mut parser = lexopt::Parser::from_args(args);

    while let Some(arg) = parser.next()? {
        if let Long("help") | Short('h') = arg {
            println!(
                "Usage: whoami\nPrint the user name associated with the current effective user ID"
            );
            return Ok(ExitCode::SUCCESS);
        } else {
            return Err(arg.unexpected().into());
        }
    }

    let uid = geteuid();

    let mut parser = Parser::new()?;

    loop {
        if let Some(entry) = parser.next_entry()? {
            if entry.uid == uid {
                let mut lock = stdout().lock();
                writeln!(lock, "{}", entry.name);
                break;
            }
        } else {
            eprintln!("cannot find name for user ID {}", uid.as_raw());
            break;
        }
    }

    return Ok(ExitCode::SUCCESS);
}
