use proc_macro2::{Ident, TokenStream};
use syn::{parse2, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma};

// TODO drop the Debugs

#[derive(Debug)]
struct NestedAttribute {
    stuff: Punctuated<Ident, Comma>,
}

#[derive(Debug)]
struct Attributes {
    optional_name: Option<Ident>,
    envs: Punctuated<Ident, Comma>,
}

impl Parse for Attributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        eprintln!("{:?}", input);

        let mut optional_name = None;
        let mut envs: Punctuated<Ident, Comma> = Punctuated::new();

        while !input.is_empty() {
            let starting_ident: Ident = input.parse()?;
            let _equals: Token![=] = input.parse()?;

            if starting_ident.to_string().eq("envs") {
                envs = Punctuated::<Ident, Comma>::parse_terminated(input)?;
            } else if starting_ident.to_string().eq("name") {
                optional_name = Some(input.parse()?);
            }
        }

        Ok(Attributes {
            optional_name,
            envs,
        })
    }
}

#[derive(Clone)]
pub enum EnvSetting {
    None,
    Env(Vec<String>)
}

pub fn get_environments(attributes: TokenStream) -> EnvSetting {
    let possible_attributes: Result<Attributes, syn::Error> = parse2(attributes);

    possible_attributes
        .map(|a| a.envs)
        .map(|env| env.iter().map(|v| v.to_string()).collect())
        .map(|v: Vec<String>| {
            if v.is_empty() {
                EnvSetting::None
            } else {
                EnvSetting::Env(v)
            }
        })
        .unwrap_or_else(|_| EnvSetting::None)
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use super::*;
    use syn::token::{Eq};
    use quote::{quote, ToTokens};

    #[test]
    fn get_environments_should_return_all_present_envs() {
        let mut stream = TokenStream::new();
        let mut env_with_equals: Punctuated<Ident, Eq> = Punctuated::new();
        env_with_equals.push(Ident::new("envs", Span::call_site()));
        env_with_equals.push_punct(Eq::default());
        env_with_equals.to_tokens(&mut stream);

        let mut envs_separated_by_comma: Punctuated<Ident, Comma> = Punctuated::new();
        envs_separated_by_comma.push(Ident::new("dev", Span::call_site()));
        envs_separated_by_comma.push_punct(Comma::default());
        envs_separated_by_comma.push(Ident::new("prod", Span::call_site()));
        envs_separated_by_comma.to_tokens(&mut stream);

        let actual = get_environments(stream);

        match actual {
            EnvSetting::Env(actual_vec) => {
                assert_eq!(actual_vec.len(), 2);
                assert_eq!(actual_vec[0], "dev".to_string());
                assert_eq!(actual_vec[1], "prod".to_string());
            }
            EnvSetting::None => panic!("Expected ENVS"),
        }
    }

    #[test]
    fn get_environments_should_return_empty_when_for_empty_input() {
        let stream = TokenStream::new();

        let actual = get_environments(stream);

        match actual {
            EnvSetting::None => {}
            EnvSetting::Env(e) => panic!("Expected NONE for env but got {:?}", e),
        }
    }
}