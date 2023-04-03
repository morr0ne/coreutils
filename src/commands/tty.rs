use std::{env::Args, io::Write, os::unix::io::AsRawFd, process::ExitCode};

use nix::unistd::ttyname;

use crate::{util::new_command, Result};

pub fn tty(args: Args) -> Result {
    new_command(
        "tty",
        "Print the file name of the terminal connected to standard input",
        
    )
    .get_matches_from(args);

    let mut stdout = std::io::stdout();
    let raw_fd = stdout.as_raw_fd();

    let name = ttyname(raw_fd);

    match name {
        Ok(name) => {
            writeln!(stdout, "{}", name.display())?;
            Ok(ExitCode::SUCCESS)
        }
        Err(_) => {
            writeln!(stdout, "not a tty")?;
            Ok(ExitCode::from(1))
        }
    }
}
