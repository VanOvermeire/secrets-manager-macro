use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::{LitStr, Token};

#[derive(Debug)] // TODO remove?
pub struct Input {
    _secret_prefix: Ident,
    _colon: Token!(:),
    pub secret_name: LitStr,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Input {
            _secret_prefix: input.parse().unwrap(),
            _colon: input.parse().unwrap(),
            secret_name: input.parse().unwrap(),
        })
    }
}
