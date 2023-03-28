use std::collections::HashMap;
use proc_macro2::{Ident, TokenStream};
use syn::{Error, ItemStruct, parse2};
use syn::spanned::Spanned;

use crate::implementation::aws::{secret_manager};
use crate::implementation::errors::RetrievalError;
use crate::implementation::input::{EnvSetting, get_environments};
use crate::implementation::output::create_output;
use crate::implementation::transformations::{keys_as_ident_list, possible_base_names};

fn retrieve_real_name_and_keys(base_secret_names: Vec<String>, env_setting: EnvSetting) -> Result<(String, HashMap<String, String>), RetrievalError> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(secret_manager(base_secret_names, env_setting))
}

pub fn create_secret_manager(attributes: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemStruct = match parse2(item.clone()) {
        Ok(it) => it,
        Err(_) => return Error::new(
            item.span(),
            "Invalid input received. Expected an empty struct",
        ).into_compile_error(),
    };

    let env_setting = get_environments(attributes);

    let secret_struct_name = input.ident.to_string();
    let possible_names = possible_base_names(&secret_struct_name);

    match retrieve_real_name_and_keys(possible_names, env_setting) {
        Ok((actual_base_secret_name, key_map)) => {
            let keys: Vec<Ident> = keys_as_ident_list(key_map);
            create_output(&input, &keys, &actual_base_secret_name)
        }
        Err(e) => e.into_compile_error(input.ident.span())
    }
}
