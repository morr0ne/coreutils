use std::{
    env::Args,
    ffi::CStr,
    io::{stdout, Write},
    process::ExitCode,
};

use clap::{Arg, ArgAction};

use crate::{
    util::{get_uname, new_command},
    Result,
};

pub fn uname(args: Args, multicall: bool) -> Result {
    let matches = new_command("uname", "Print system informations", multicall)
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("kernel-name")
                .short('s')
                .long("kernel-name")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("nodename")
                .short('n')
                .long("nodename")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("kernel-release")
                .short('r')
                .long("kernel-release")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("kernel-version")
                .short('v')
                .long("kernel-version")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("machine")
                .short('m')
                .long("machine")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("processor")
                .short('p')
                .long("processor")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("hardware-platform")
                .short('i')
                .long("hardware-platform")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("operating-system")
                .short('o')
                .long("operating-system")
                .action(ArgAction::SetTrue),
        )
        .get_matches_from(args);

    let utsname = get_uname()?;
    let mut lock = stdout().lock();

    let all = matches.get_flag("all");

    if matches.get_flag("kernel-name") || all {
        let sysname = unsafe { CStr::from_ptr(utsname.sysname.as_ptr()) };
        lock.write_all(sysname.to_bytes())?;
        lock.write_all(b" ")?;
    }

    if matches.get_flag("nodename") || all {
        let nodename = unsafe { CStr::from_ptr(utsname.nodename.as_ptr()) };
        lock.write_all(nodename.to_bytes())?;
        lock.write_all(b" ")?;
    }

    if matches.get_flag("kernel-release") || all {
        let release = unsafe { CStr::from_ptr(utsname.release.as_ptr()) };
        lock.write_all(release.to_bytes())?;
        lock.write_all(b" ")?;
    }

    if matches.get_flag("kernel-version") || all {
        let version = unsafe { CStr::from_ptr(utsname.version.as_ptr()) };
        lock.write_all(version.to_bytes())?;
        lock.write_all(b" ")?;
    }

    if matches.get_flag("machine") || all {
        let machine = unsafe { CStr::from_ptr(utsname.machine.as_ptr()) };
        lock.write_all(machine.to_bytes())?;
        lock.write_all(b" ")?;
    }

    if matches.get_flag("processor") {
        lock.write_all(b"unknown")?;
        lock.write_all(b" ")?;
    }

    if matches.get_flag("hardware-platform") {
        lock.write_all(b"unknown")?;
        lock.write_all(b" ")?;
    }

    if matches.get_flag("operating-system") || all {
        lock.write_all(b"GNU/Linux")?; // TODO: this obv should depend on the current os
    }

    lock.write_all(b"\n")?;

    Ok(ExitCode::SUCCESS)
}
