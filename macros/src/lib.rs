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
