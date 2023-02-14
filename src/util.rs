use clap::{crate_version, Command};

pub fn new_command(name: &'static str, about: &'static str, multicall: bool) -> Command {
    Command::new(name)
        .version(crate_version!())
        .about(about)
        // .override_usage(name)
        .no_binary_name(multicall)
}

#[macro_export]
macro_rules! define_command_bin {
    ($command:ident) => {
        fn main() -> ::coreutils::Result {
            ::coreutils::commands::$command(::std::env::args(), false)
        }
    };
}
