use coreutils::{commands, Error, Result};
use coreutils_macros::define_commands;

fn main() -> Result {
    let mut args = std::env::args();

    if let Some(arg) = args.nth(1) {
        define_commands!(
            arg, "arch", "b2sum", "base32", "base64", "basename", "basenc", "tty", "uname",
            "uptime", "whoami", "yes"
        )
    } else {
        Err(Error::NoCommand)
    }
}
