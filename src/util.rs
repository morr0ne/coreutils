use std::{io::Error as IoError, mem::MaybeUninit};

use clap::{crate_version, Command};
use libc::{uname, utsname};

pub fn new_command(name: &'static str, about: &'static str, multicall: bool) -> Command {
    Command::new(name)
        .version(crate_version!())
        .about(about)
        // .override_usage(name)
        .no_binary_name(multicall)
}

// #[cfg(feature = "libc")]
pub fn get_uname() -> Result<utsname, IoError> {
    let mut utsname = MaybeUninit::<utsname>::uninit();

    if unsafe { uname(utsname.as_mut_ptr()) } == 0 {
        Ok(unsafe { utsname.assume_init() })
    } else {
        Err(IoError::last_os_error())
    }
}
