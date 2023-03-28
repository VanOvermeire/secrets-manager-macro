use proc_macro2::{Ident, TokenStream};
use syn::{parse2, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma};

#[derive(Debug)]
struct Attributes {
    env: Punctuated<Ident, Comma>,
}

impl Parse for Attributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _env_ident: Ident = input.parse()?;
        let _equals: Token![=] = input.parse()?;

        Ok(Attributes {
            env: Punctuated::<Ident, Comma>::parse_terminated(input)?
        })
    }
}

pub enum EnvSetting {
    NONE,
    ENVS(Vec<String>)
}

pub fn get_environments(attributes: TokenStream) -> EnvSetting {
    let possible_attributes: Result<Attributes, syn::Error> = parse2(attributes);

    possible_attributes
        .map(|a| a.env)
        .map(|env| env.iter().map(|v| v.to_string()).collect())
        .map(EnvSetting::ENVS)
        .unwrap_or_else(|_| EnvSetting::NONE)
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use super::*;
    use syn::token::{Eq};
    use quote::ToTokens;

    #[test]
    fn get_environments_should_return_all_present_envs() {
        let mut stream = TokenStream::new();
        let mut env_with_equals: Punctuated<Ident, Eq> = Punctuated::new();
        env_with_equals.push(Ident::new("env", Span::call_site()));
        env_with_equals.push_punct(Eq::default());
        env_with_equals.to_tokens(&mut stream);

        let mut envs_separated_by_comma: Punctuated<Ident, Comma> = Punctuated::new();
        envs_separated_by_comma.push(Ident::new("dev", Span::call_site()));
        envs_separated_by_comma.push_punct(Comma::default());
        envs_separated_by_comma.push(Ident::new("prod", Span::call_site()));
        envs_separated_by_comma.to_tokens(&mut stream);

        let actual = get_environments(stream);

        match actual {
            EnvSetting::ENVS(actual_vec) => {
                assert_eq!(actual_vec.len(), 2);
                assert_eq!(actual_vec[0], "dev".to_string());
                assert_eq!(actual_vec[1], "prod".to_string());
            }
            EnvSetting::NONE => panic!("Expected ENVS"),
        }
    }

    #[test]
    fn get_environments_should_return_empty_when_for_empty_input() {
        let stream = TokenStream::new();

        let actual = get_environments(stream);

        match actual {
            EnvSetting::NONE => {}
            EnvSetting::ENVS(_) => panic!("Expected NONE for env"),
        }
    }
}