use std::{
    env::Args,
    ffi::CStr,
    io::{stdout, Write},
    process::ExitCode,
};

use crate::{
    util::{get_uname, new_command},
    Result,
};

pub fn arch(args: Args, multicall: bool) -> Result {
    new_command("arch", "Prints the machine architecture", multicall).get_matches_from(args);

    let utsname = get_uname()?;
    let machine = unsafe { CStr::from_ptr(utsname.machine.as_ptr()) };

    let mut lock = stdout().lock();
    lock.write_all(machine.to_bytes())?;
    lock.write_all(b"\n")?;

    Ok(ExitCode::SUCCESS)
}
