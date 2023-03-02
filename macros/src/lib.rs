use std::{env, fs, path::PathBuf};

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, Ident, LitStr, Token};

struct Input {
    matcher: Ident,
    _separator: Token![,],
    commands: Punctuated<LitStr, Option<Token![,]>>,
}

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Input {
            matcher: input.parse()?,
            _separator: input.parse()?,
            commands: Punctuated::parse_terminated(input)?,
        })
    }
}

#[proc_macro]
pub fn define_commands(tokens: TokenStream) -> TokenStream {
    let Input {
        matcher, commands, ..
    } = parse_macro_input!(tokens);

    let mut fields = Vec::new();

    for command in commands {
        let command_function = format_ident!("{}", command.value());
        fields.push(quote! {
            #[cfg(feature = #command)]
            #command => commands::#command_function(args, true),
        })
    }

    quote! {
        match #matcher.as_str() {
            #(#fields)*
            command => Err(Error::UnknownCommand(command.to_string())),
        }
    }
    .into()
}

#[proc_macro]
pub fn import_commands(tokens: TokenStream) -> TokenStream {
    let path = parse_macro_input!(tokens as LitStr).value();

    let path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Failed to find Cargo.toml"))
        .join(path);

    let mut commands = Vec::new();

    for command in fs::read_dir(path).expect("Failed to read path") {
        let command_path = command.expect("Failed to read command").path();
        let command_module = command_path.file_stem().unwrap();

        let mut module_name = command_module.to_str().unwrap().to_string();

        if module_name == "mod" {
            continue;
        }

        if module_name == "true" || module_name == "false" {
            module_name = format!("r#{module_name}")
        }

        let module_ident = format_ident!("{}", module_name);
        commands.push(quote! {
            #[cfg(feature = #module_name)]
            mod #module_ident;

            #[cfg(feature = #module_name)]
            pub use #module_ident::#module_ident;
        })
    }

    quote! {
        #(#commands)*
    }
    .into()
}
