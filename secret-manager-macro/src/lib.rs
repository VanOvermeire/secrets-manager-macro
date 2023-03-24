mod implementation;

use crate::implementation::entrypoint::{create_secret_manager, create_secret_manager_new};
use proc_macro::TokenStream;

#[proc_macro]
pub fn build_secrets_struct(item: TokenStream) -> TokenStream {
    create_secret_manager(item.into()).into()
}

#[proc_macro_attribute]
pub fn build_secrets_structz(attr: TokenStream, item: TokenStream) -> TokenStream {
    create_secret_manager_new(attr.into(), item.into()).into()
}
