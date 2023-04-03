use std::{
    env::Args,
    io::{stdout, Write},
    process::ExitCode,
};

use rustix::process::uname;

use crate::{util::new_command, Result};

pub fn arch(args: Args) -> Result {
    new_command("arch", "Prints the machine architecture").get_matches_from(args);

    let mut lock = stdout().lock();
    lock.write_all(uname().machine().to_bytes())?;
    lock.write_all(b"\n")?;

    Ok(ExitCode::SUCCESS)
}
