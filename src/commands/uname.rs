use std::{
    env::Args,
    ffi::CStr,
    io::{self, stdout, Write},
    process::ExitCode,
};

use clap::{Arg, ArgAction};

use crate::{
    util::{get_uname, new_command},
    Error, Result,
};

pub fn uname(args: Args, multicall: bool) -> Result {
    let matches = new_command("uname", "Print system informations", multicall)
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .action(ArgAction::SetTrue),
        )
        .get_matches_from(args);

    // TODO: Handle args

    if let Some(utsname) = get_uname() {
        let sysname = unsafe { CStr::from_ptr(utsname.sysname.as_ptr()) };
        let nodename = unsafe { CStr::from_ptr(utsname.nodename.as_ptr()) };
        let release = unsafe { CStr::from_ptr(utsname.release.as_ptr()) };
        let version = unsafe { CStr::from_ptr(utsname.version.as_ptr()) };
        let machine = unsafe { CStr::from_ptr(utsname.machine.as_ptr()) };

        let mut lock = stdout().lock();
        lock.write_all(sysname.to_bytes())?;
        lock.write_all(b" ")?;
        lock.write_all(nodename.to_bytes())?;
        lock.write_all(b" ")?;
        lock.write_all(release.to_bytes())?;
        lock.write_all(b" ")?;
        lock.write_all(version.to_bytes())?;
        lock.write_all(b" ")?;
        lock.write_all(machine.to_bytes())?;
        lock.write_all(b" ")?;
        lock.write_all(b"GNU/Linux")?; // TODO: this obv should depend on the current os
        lock.write_all(b"\n")?;

        Ok(ExitCode::SUCCESS)
    } else {
        Err(Error::IoError(io::Error::last_os_error()))
    }
}
