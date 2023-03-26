mod implementation;

use crate::implementation::entrypoint::{create_secret_manager};
use proc_macro::{TokenStream};
use syn::parse::{Parse};

#[proc_macro_attribute]
pub fn build_secrets_struct(attr: TokenStream, item: TokenStream) -> TokenStream {
    create_secret_manager(attr.into(), item.into()).into()
}
