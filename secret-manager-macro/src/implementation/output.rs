use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub fn create_secret_struct(keys: &Vec<Ident>) -> TokenStream {
    let secret_fields = keys.iter().map(|k| quote!(pub #k: SecretString));

    quote! {
        #[derive(Debug)]
        struct Secrets {
            #(#secret_fields,)*
        }
    }
}

pub fn create_init_for_secrets(keys: &Vec<Ident>, actual_secret_name: &str) -> TokenStream {
    let init_of_struct = keys.iter().map(|k| {
        quote! {
            #k: SecretString::new(map.get(stringify!(#k)).unwrap().to_string())
        }
    });
    let secret_name = Ident::new(actual_secret_name, Span::call_site());

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

        impl Secrets {
            pub async fn new() -> Self {
                let shared_config = aws_config::from_env().load().await;
                let client = aws_sdk_secretsmanager::Client::new(&shared_config);
                let secret_value = get_secret(&client, &stringify!(#secret_name)).await;
                let map = get_secret_as_map(secret_value);

                Secrets {
                    #(#init_of_struct,)*
                }
            }
        }
    }
}
