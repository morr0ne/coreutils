use std::{
    env::Args,
    ffi::CStr,
    io::{stdout, Write},
    process::ExitCode,
};

use rustix::process::geteuid;

use crate::{util::new_command, Result};

pub fn whoami(args: Args, multicall: bool) -> Result {
    new_command(
        "whoami",
        "Print the user name associated with the current effective user ID",
        multicall,
    )
    .get_matches_from(args);

    let uid = geteuid();
    let pw = unsafe { libc::getpwuid(uid.as_raw()) };

    if pw.is_null() {
        eprintln!("cannot find name for user ID {}", uid.as_raw());
    } else {
        let name = unsafe { CStr::from_ptr((*pw).pw_name) };

        let mut lock = stdout().lock();
        lock.write_all(name.to_bytes())?;
        lock.write_all(b"\n")?;
    }

    Ok(ExitCode::SUCCESS)
}
