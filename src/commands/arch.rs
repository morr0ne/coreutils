use std::{
    env::Args,
    io::{Write, stdout},
    process::ExitCode,
};

use lexopt_derive::Parser;
use rustix::system::uname;

use crate::Result;

#[derive(Parser)]
struct ParsedArgs {}

pub fn arch(args: Args) -> Result {
    ParsedArgs::parse(args, "Usage: arch\nPrints the machine architecture")?;

    let mut lock = stdout().lock();
    lock.write_all(uname().machine().to_bytes())?;
    lock.write_all(b"\n")?;

    Ok(ExitCode::SUCCESS)
}
