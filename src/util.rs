use clap::{crate_version, Command};

pub fn new_command(name: &'static str, about: &'static str) -> Command {
    Command::new(name).version(crate_version!()).about(about)
}
