use std::collections::HashMap;
use proc_macro2::{Ident, TokenStream};
use syn::{Error, ItemStruct, parse2};
use syn::spanned::Spanned;

use crate::implementation::aws;
use crate::implementation::errors::RetrievalError;
use crate::implementation::input::{self, EnvSetting};
use crate::implementation::output;
use crate::implementation::transformations;

// TODO make this one async, use it to combine stuff, and create the runtime in below method
fn retrieve_real_name_and_keys(base_secret_names: Vec<String>, env_setting: EnvSetting) -> Result<(String, HashMap<String, String>), RetrievalError> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(aws::secret_manager(base_secret_names, env_setting))
}

pub fn create_secret_manager(attributes: TokenStream, item: TokenStream) -> TokenStream {
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

    match retrieve_real_name_and_keys(possible_names, env_setting.clone()) {
        Ok((actual_base_secret_name, key_map)) => {
            let keys: Vec<Ident> = transformations::keys_as_ident_list(key_map);
            output::create_output(&input, &keys, &actual_base_secret_name, &env_setting)
        }
        Err(e) => e.into_compile_error(input.ident.span())
    }
}
