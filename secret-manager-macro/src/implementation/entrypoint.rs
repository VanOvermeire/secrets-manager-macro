use std::collections::HashMap;

use proc_macro2::{Ident, Span, TokenStream};
use syn::{Error, ItemStruct, parse2};
use syn::spanned::Spanned;

use crate::implementation::aws::retrieve_real_name_and_keys;
use crate::implementation::output::{create_output_new};

fn keys_as_ident_list(key_map: HashMap<String, String>) -> Vec<Ident> {
    key_map
        .keys()
        .map(|k| Ident::new(k, Span::call_site()))
        .collect()
}

pub fn create_secret_manager_new(_: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemStruct = match parse2(item.clone()) {
        Ok(it) => it,
        Err(_) => return Error::new(
            item.span(),
            "Invalid input received. Expected an empty struct",
        ).into_compile_error(),
    };

    match retrieve_real_name_and_keys(&input.ident.to_string()) {
        Ok((actual_secret_name, key_map)) => {
            let keys: Vec<Ident> = keys_as_ident_list(key_map);
            create_output_new(input, &keys, &actual_secret_name)
        }
        Err(e) => return e.into_compile_error(input.ident.span())
    }
}
