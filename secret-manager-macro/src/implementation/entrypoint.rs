use proc_macro2::{Ident, TokenStream};
use syn::{Error, ItemStruct, parse2};
use syn::spanned::Spanned;

use crate::implementation::aws::retrieve_real_name_and_keys;
use crate::implementation::output::create_output;
use crate::implementation::transformations::{keys_as_ident_list, possible_base_names};

pub fn create_secret_manager(_: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemStruct = match parse2(item.clone()) {
        Ok(it) => it,
        Err(_) => return Error::new(
            item.span(),
            "Invalid input received. Expected an empty struct",
        ).into_compile_error(),
    };

    let secret_struct_name = input.ident.to_string();
    let possible_names = possible_base_names(&secret_struct_name);

    match retrieve_real_name_and_keys(possible_names) {
        Ok((actual_secret_name, key_map)) => {
            let keys: Vec<Ident> = keys_as_ident_list(key_map);
            create_output(&input, &keys, &actual_secret_name) // a lot of parameters, maybe create helper struct
        }
        Err(e) => e.into_compile_error(input.ident.span())
    }
}
