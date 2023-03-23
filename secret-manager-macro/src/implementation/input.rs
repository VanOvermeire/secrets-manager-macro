use syn::parse::{Parse, ParseStream};
use syn::LitStr;

#[derive(Debug)]
pub struct Input {
    pub secret_name: LitStr,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Input {
            secret_name: input.parse().unwrap(),
        })
    }
}
