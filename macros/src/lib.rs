use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, Ident, Token};

struct Input {
    commands: Punctuated<Ident, Option<Token![,]>>,
}

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Input {
            commands: input.parse_terminated(Ident::parse)?,
        })
    }
}

#[proc_macro]
pub fn call_commands(tokens: TokenStream) -> TokenStream {
    let Input { commands } = parse_macro_input!(tokens);

    let mut fields = Vec::new();

    for command in commands {
        let lit = command.to_string();
        fields.push(quote! {
            #[cfg(feature = #lit)]
            #lit => commands::#command(args, true)
        })
    }
    // #(#fields),*

    quote! {

         "whoami" => commands::whoami(args, true)
    }
    .into()
}
