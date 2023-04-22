use std::{
    env::Args,
    ffi::CStr,
    io::{stdout, Write},
    process::ExitCode,
};

use lexopt::prelude::*;
use rustix::process::geteuid;

use crate::Result;

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
