use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::ItemStruct;

// there are libraries for this, but this way no additional import is needed (there are enough already)
fn create_secret_string_struct() -> TokenStream {
    quote! {
        #[derive(Clone,PartialEq)]
        pub struct SecretString(String);

        impl SecretString {
            pub fn new(input: String) -> Self {
                SecretString(input)
            }
        }

        impl std::fmt::Debug for SecretString {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&format!("{}****", self.0.chars().next().unwrap_or('*')))
            }
        }

        impl AsRef<str> for SecretString {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }
    }
}

fn create_init_for_secrets_new(keys: &Vec<Ident>, secret_struct_name: &Ident, actual_secret_name: &str) -> TokenStream {
    let init_of_struct = keys.iter().map(|k| {
        quote! {
            #k: SecretString::new(map.get(stringify!(#k)).unwrap().to_string())
        }
    });

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
                .unwrap()
        }

        fn get_secret_as_map(
            output: aws_sdk_secretsmanager::output::GetSecretValueOutput,
        ) -> std::collections::HashMap<String, String> {
            let content = output
                .secret_string()
                .map_or_else(|| "{}".to_string(), |v| v.to_string());
            serde_json::from_str(&content).unwrap()
        }

        impl #secret_struct_name {
            pub async fn new() -> Self {
                let shared_config = aws_config::from_env().load().await;
                let client = aws_sdk_secretsmanager::Client::new(&shared_config);
                let secret_value = get_secret(&client, #actual_secret_name).await;
                let map = get_secret_as_map(secret_value);

                #secret_struct_name {
                    #(#init_of_struct,)*
                }
            }
        }
    }
}

pub fn create_output_new(item: ItemStruct, keys: &Vec<Ident>, actual_secret_name: &str) -> TokenStream {
    let name = item.ident;
    let attributes = item.attrs;

    let secret_string_struct = create_secret_string_struct();
    let secret_fields = keys.iter().map(|k| quote!(pub #k: SecretString));
    let new = create_init_for_secrets_new(keys, &name, actual_secret_name);

    quote!(
        #secret_string_struct

        #(#attributes)*
        pub struct #name {
            #(#secret_fields,)*
        }

        #new
    )
}