use crate::implementation::aws::retrieve_keys;
use crate::implementation::input::Input;
use crate::implementation::output::{create_init_for_secrets, create_secret_struct};
use crate::implementation::secret_string::create_secret_string_struct;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::HashMap;
use std::iter::Map;
use std::slice::Iter;
use syn::parse::Parse;
use syn::parse2;

// TODO avoid string allocations
pub fn create_secret_manager(item: TokenStream) -> TokenStream {
    // eprintln!("{:?}", item);
    let input: Input = parse2(item).unwrap(); // TODO secret-name: "some secret" as input
    let key_map = retrieve_keys(&input.secret_name.value()).unwrap(); // TODO no unwrap, handle errors

    // remove
    // let mut key_map = HashMap::new();
    // key_map.insert("key1", "value1");
    // key_map.insert("key2", "value2");

    let keys: Vec<Ident> = key_map
        .keys()
        .map(|k| Ident::new(k, Span::call_site()))
        .collect();

    let secret_struct = create_secret_struct(&keys);
    let secret_string_struct = create_secret_string_struct();
    let new = create_init_for_secrets(&keys, &"SampleSecrets"); // TODO -> actual secret...

    quote! {
        #secret_string_struct

        #secret_struct

        #new
    }
}
