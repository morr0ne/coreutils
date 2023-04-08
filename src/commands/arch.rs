use std::{
    env::Args,
    io::{stdout, Write},
    process::ExitCode,
};

use lexopt::prelude::*;
use rustix::process::uname;

use crate::Result;

pub fn arch(args: Args) -> Result {
    let mut parser = lexopt::Parser::from_args(args);

    while let Some(arg) = parser.next()? {
        if let Long("help") = arg {
            println!("Usage: arch\nPrints the machine architecture");
            return Ok(ExitCode::SUCCESS);
        } else {
            return Err(arg.unexpected().into());
        }
    }

    let mut lock = stdout().lock();
    lock.write_all(uname().machine().to_bytes())?;
    lock.write_all(b"\n")?;

    Ok(ExitCode::SUCCESS)
}
