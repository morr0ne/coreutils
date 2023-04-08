use std::{
    env::Args,
    io::{stdout, Write},
    process::ExitCode,
};

use lexopt::prelude::*;
use rustix::{io::stdout as rustix_stdout, termios::ttyname};

use crate::Result;

pub fn tty(args: Args) -> Result {
    let mut parser = lexopt::Parser::from_args(args);

    let mut silent = false;

    while let Some(arg) = parser.next()? {
        match arg {
            Short('s') | Long("silent") | Long("quiet") => silent = true,
            Long("help") => {
                println!("Usage: yes");
                return Ok(ExitCode::SUCCESS);
            }
            _ => return Err(arg.unexpected().into()),
        }
    }

    let name = ttyname(rustix_stdout(), Vec::new());

    let mut lock = stdout().lock();

    match name {
        Ok(name) => {
            if !silent {
                lock.write_all(name.to_bytes())?;
                lock.write_all(b"\n")?;
            }

            Ok(ExitCode::SUCCESS)
        }
        Err(_) => {
            if !silent {
                writeln!(lock, "not a tty")?;
            }

            Ok(ExitCode::FAILURE)
        }
    }
}
