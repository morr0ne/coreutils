use std::fs;

const COMMANDS: &[&str] = &[
    "arch", "basename", "false", "true", "tty", "who", "whoami", "yes",
];

fn main() {
    fs::create_dir_all("bin").expect("Failed to create bin folder");

    for command in COMMANDS {
        fs::write(
            format!("bin/{command}.rs"),
            format!("coreutils::define_command_bin!({command});"),
        )
        .expect("Failed to write file");
    }
}
