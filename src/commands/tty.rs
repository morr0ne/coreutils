use std::{
    env::Args,
    io::{stdout, Write},
    process::ExitCode,
};

use rustix::{io::stdout as rustix_stdout, termios::ttyname};

use crate::{util::new_command, Result};

pub fn tty(args: Args) -> Result {
    new_command(
        "tty",
        "Print the file name of the terminal connected to standard input",
    )
    .get_matches_from(args);

    let name = ttyname(rustix_stdout(), Vec::new());

    let mut lock = stdout().lock();

    match name {
        Ok(name) => {
            lock.write_all(name.to_bytes())?;
            lock.write_all(b"\n")?;

            Ok(ExitCode::SUCCESS)
        }
        Err(_) => {
            writeln!(lock, "not a tty")?;
            Ok(ExitCode::from(1))
        }
    }
}
