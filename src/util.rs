use clap::{crate_version, Command};

pub fn new_command(name: &'static str, about: &'static str, multicall: bool) -> Command {
    Command::new(name)
        .version(crate_version!())
        .about(about)
        // .override_usage(name)
        .no_binary_name(multicall)
}
