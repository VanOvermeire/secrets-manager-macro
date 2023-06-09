use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::ItemStruct;
use crate::implementation::input::EnvSetting;

// there are libraries for secret strings, but this way no additional import is needed (there are enough already)
fn create_secret_string_struct(secret_string_name: &Ident) -> TokenStream {
    quote! {
        #[derive(Clone,PartialEq)]
        pub struct #secret_string_name(String);

        impl #secret_string_name {
            pub fn new(input: String) -> Self {
                #secret_string_name(input)
            }
        }

        impl std::fmt::Debug for #secret_string_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&format!("{}****", self.0.chars().next().unwrap_or('*')))
            }
        }

        impl AsRef<str> for #secret_string_name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }
    }
}

fn get_secret_string_name(secret_struct_name: &Ident) -> Ident {
    format_ident!("{}SecretString", secret_struct_name.to_string())
}

fn create_init_for_secrets(keys: &[Ident], secret_struct_name: &Ident, actual_base_secret_name: &str, env_setting: &EnvSetting) -> TokenStream {
    let secret_string_name = get_secret_string_name(secret_struct_name);

    let init_of_struct = keys.iter().map(|k| {
        quote! {
            #k: #secret_string_name::new(map.get(stringify!(#k)).expect(&format!("Expected key {} to be present", stringify!(#k))).to_string())
        }
    });

    let build_secret_name = match env_setting {
        EnvSetting::None => {
            quote! {
                let secret_name = #actual_base_secret_name;
            }
        }
        EnvSetting::Env(_) => {
            quote! {
                let env_vec: Vec<String> = ["ENV", "ENVIRONMENT"].iter().flat_map(std::env::var).collect();
                let secret_name = env_vec
                    .first()
                    .map(|env| format!("/{}/{}", env, #actual_base_secret_name))
                    .expect("Expected environment variable 'ENV' or 'ENVIRONMENT' to be present");
            }
        }
    };

    quote! {
        async fn get_secret(
                client: &aws_sdk_secretsmanager::Client,
                secret_name: &str,
        ) -> aws_sdk_secretsmanager::output::GetSecretValueOutput {
            client
                .get_secret_value()
                .secret_id(secret_name)
                .send()
                .await
                .expect(&format!("Expected to find secret with name {}. Instead got error", secret_name))
        }

        fn get_secret_as_map(
            output: aws_sdk_secretsmanager::output::GetSecretValueOutput,
        ) -> std::collections::HashMap<String, String> {
            let content = output
                .secret_string()
                .map_or_else(|| "{}".to_string(), |v| v.to_string());
            serde_json::from_str(&content).expect("Expected to be able to parse the secret value")
        }

        impl #secret_struct_name {
            pub async fn new() -> Self {
                let shared_config = aws_config::from_env().load().await;
                let client = aws_sdk_secretsmanager::Client::new(&shared_config);

                #build_secret_name

                let secret_value = get_secret(&client, &secret_name).await;
                let map = get_secret_as_map(secret_value);

                #secret_struct_name {
                    #(#init_of_struct,)*
                }
            }
        }
    }
}

pub fn create_output(item: &ItemStruct, keys: &[Ident], actual_base_secret_name: &str, env_setting: &EnvSetting) -> TokenStream {
    let name = &item.ident;
    let attributes = &item.attrs;

    let secret_string_name = get_secret_string_name(name);
    let secret_string_struct = create_secret_string_struct(&secret_string_name);
    let secret_fields = keys.iter().map(|k| quote!(pub #k: #secret_string_name));
    let new_impl_block = create_init_for_secrets(keys, name, actual_base_secret_name, env_setting);

    quote!(
        #secret_string_struct

        #(#attributes)*
        pub struct #name {
            #(#secret_fields,)*
        }

        #new_impl_block
    )
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use proc_macro2::Ident;
    use super::*;

    #[test]
    fn should_generate_ident_with_secret_string_suffix() {
        let example_ident = Ident::new("Example", Span::call_site());

        let actual = get_secret_string_name(&example_ident);

        assert_eq!(actual.to_string(), "ExampleSecretString".to_string());
    }
}