use coreutils::{Error, Result};

fn main() -> Result {
    let mut args = std::env::args();

    if let Some(arg) = args.nth(1) {
        match arg.as_str() {
            // #[cfg(feature = "arch")]
            // "arch" => coreutils::commands::arch::arch(args),
            #[cfg(feature = "b2sum")]
            "b2sum" => coreutils::commands::b2sum::b2sum(args),
            #[cfg(feature = "yes")]
            "yes" => coreutils::commands::yes::yes(args),
            _ => Err(Error::UnknownCommand(arg)),
        }
    } else {
        return Ok(::std::process::ExitCode::SUCCESS);
    }
}
