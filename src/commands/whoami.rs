use std::{
    env::Args,
    io::{Write, stdout},
    process::ExitCode,
};

use lexopt_derive::Parser;
use rustix::process::geteuid;

use crate::{Result, utils::passwd::Parser};

#[derive(Parser)]
struct ParsedArgs {}

pub fn whoami(args: Args) -> Result {
    ParsedArgs::parse(
        args,
        "Usage: whoami\nPrint the user name associated with the current effective user ID",
    )?;

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
