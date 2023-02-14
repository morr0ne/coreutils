use std::{
    env::Args,
    ffi::CStr,
    io::{self, stdout, Write},
    mem::MaybeUninit,
    process::ExitCode,
};

use libc::{uname, utsname};

use crate::{util::new_command, Error, Result};

pub fn arch(args: Args, multicall: bool) -> Result {
    new_command(
        "arch",
        "Prints the machine architecture",
        multicall,
    )
    .get_matches_from(args);

    let mut utsname = MaybeUninit::<utsname>::uninit();
    let result = unsafe { uname(utsname.as_mut_ptr()) };

    if result == 0 {
        // This is safe because uname returned 0 meaning it succeded
        let utsname = unsafe { utsname.assume_init() };
        let machine = unsafe { CStr::from_ptr(utsname.machine.as_ptr()) };

        let mut lock = stdout().lock();
        lock.write_all(machine.to_bytes())?;
        lock.write_all(b"\n")?;

        Ok(ExitCode::SUCCESS)
    } else {
        Err(Error::IoError(io::Error::last_os_error()))
    }
}
