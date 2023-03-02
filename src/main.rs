use coreutils::{Error, Result};
use coreutils_macros::call_commands;

fn main() -> Result {
    let mut args = std::env::args();

    call_commands!(args)
}
