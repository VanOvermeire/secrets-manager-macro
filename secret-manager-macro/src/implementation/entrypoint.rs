use crate::implementation::aws::retrieve_real_name_and_keys;
use crate::implementation::input::Input;
use crate::implementation::output::{create_init_for_secrets, create_secret_struct};
use crate::implementation::secret_string::create_secret_string_struct;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::parse2;

pub fn create_secret_manager(item: TokenStream) -> TokenStream {
    eprintln!("{:?}", item);
    let input: Input = parse2(item).unwrap();
    let (actual_secret_name, key_map) =
        retrieve_real_name_and_keys(&input.secret_name.value()).unwrap(); // TODO no unwrap, handle errors (also for parsing)

    let keys: Vec<Ident> = key_map
        .keys()
        .map(|k| Ident::new(k, Span::call_site()))
        .collect();

    let secret_struct = create_secret_struct(&keys);
    let secret_string_struct = create_secret_string_struct();
    let new = create_init_for_secrets(&keys, &actual_secret_name);

    quote! {
        #secret_string_struct

        #secret_struct

        #new
    }
}
