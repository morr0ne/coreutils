use std::{env, fs, path::PathBuf};

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Ident};

#[proc_macro]
pub fn call_commands(tokens: TokenStream) -> TokenStream {
    let args = parse_macro_input!(tokens as Ident);

    let commands = get_commands("src/commands");

    let mut fields = Vec::new();

    for command in commands {
        let mut command = command;

        if command == "true" || command == "false" {
            command = format!("r#{command}")
        }

        let module_ident = format_ident!("{}", command);
        fields.push(quote! {
            #[cfg(feature = #command)]
            #command => coreutils::commands::#module_ident(args, true),
        })
    }

    quote! {
        if let Some(arg) = #args.nth(1) {
            match arg.as_str() {
                #(#fields)*
                command => Err(Error::UnknownCommand(command.to_string())),
            }
        } else {
            Err(Error::NoCommand)
        }
    }
    .into()
}

#[proc_macro]
pub fn define_commands(_tokens: TokenStream) -> TokenStream {
    let commands = get_commands("src/commands");

    let mut commands_imports = Vec::new();

    for command in commands {
        let mut command = command;
        let module_path = format!("{command}.rs");

        if command == "true" || command == "false" {
            command = format!("r#{command}")
        }

        let module_ident = format_ident!("{}", command);
        commands_imports.push(quote! {
            #[cfg(feature = #command)]
            #[path = #module_path]
            mod #module_ident;

            #[cfg(feature = #command)]
            pub use #module_ident::#module_ident;
        });
    }

    quote! {
        #(#commands_imports)*
    }
    .into()
}

fn get_commands(path: &str) -> Vec<String> {
    let path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Failed to find Cargo.toml"))
        .join(path);

    let mut commands = Vec::new();

    for command in fs::read_dir(path).expect("Failed to read path") {
        let command_path = command.expect("Failed to read command").path();
        let command_module = command_path.file_stem().unwrap();

        commands.push(command_module.to_str().unwrap().to_string())
    }

    commands
}
