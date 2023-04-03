use std::{
    env::Args,
    ffi::CStr,
    io::{stdout, Write},
    process::ExitCode,
};

use clap::{Arg, ArgAction};
use rustix::process::uname as rustix_uname;

use crate::{util::new_command, Result};

// TODO: Default to print to kernel-name
// TODO: Trim trailing space

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

    let uname = rustix_uname();
    let mut lock = stdout().lock();

    let all = matches.get_flag("all");

    if matches.get_flag("kernel-name") || all {
        lock.write_all(uname.sysname().to_bytes())?;
        lock.write_all(b" ")?;
    }

    if matches.get_flag("nodename") || all {
        lock.write_all(uname.nodename().to_bytes())?;
        lock.write_all(b" ")?;
    }

    if matches.get_flag("kernel-release") || all {
        lock.write_all(uname.release().to_bytes())?;
        lock.write_all(b" ")?;
    }

    if matches.get_flag("kernel-version") || all {
        lock.write_all(uname.version().to_bytes())?;
        lock.write_all(b" ")?;
    }

    if matches.get_flag("machine") || all {
        lock.write_all(uname.machine().to_bytes())?;
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
