use std::collections::HashMap;
use proc_macro2::{Ident, TokenStream};
use syn::{Error, ItemStruct, parse2};
use syn::spanned::Spanned;

use crate::implementation::aws::SecretsManagerClient;
use crate::implementation::errors::RetrievalError;
use crate::implementation::input::{self, EnvSetting};
use crate::implementation::output;
use crate::implementation::transformations;
use crate::implementation::transformations::ValidatedSecrets;

async fn retrieve_real_name_and_keys(base_secret_names: Vec<String>, env_setting: EnvSetting) -> Result<(String, HashMap<String, String>), RetrievalError> {
    let client = SecretsManagerClient::new().await;
    let found_secret_names = client.get_filtered_secret_list(base_secret_names, &env_setting).await?;

    let validated_secrets = ValidatedSecrets::new(found_secret_names, env_setting)?;
    let (full_secret_name, actual_base_name) = validated_secrets.get_full_and_base_secret();

    let secret_value = client.get_secret_as_map(&full_secret_name).await?;
    Ok((actual_base_name, secret_value))
}

pub fn create_secrets_manager(attributes: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemStruct = match parse2(item.clone()) {
        Ok(it) => it,
        Err(_) => return Error::new(
            item.span(),
            "Invalid input received. Expected an empty struct",
        ).into_compile_error(),
    };

    let env_setting = input::get_environments(attributes);

    let secret_struct_name = input.ident.to_string();
    let possible_names = transformations::possible_base_names(&secret_struct_name);

    let rt = tokio::runtime::Runtime::new().unwrap();

    match rt.block_on(retrieve_real_name_and_keys(possible_names, env_setting.clone())) {
        Ok((actual_base_secret_name, key_map)) => {
            let keys: Vec<Ident> = transformations::keys_as_ident_list(key_map);
            output::create_output(&input, &keys, &actual_base_secret_name, &env_setting)
        }
        Err(e) => e.into_compile_error(input.ident.span())
    }
}
