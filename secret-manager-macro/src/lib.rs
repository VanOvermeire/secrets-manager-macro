#![doc = include_str!("../README.md")]

mod implementation;

use crate::implementation::entrypoint::{create_secret_manager};
use proc_macro::{TokenStream};

/// Will fill the given struct with secrets retrieved from AWS Secret Manager
#[proc_macro_attribute]
pub fn build_secrets_struct(attr: TokenStream, item: TokenStream) -> TokenStream {
    create_secret_manager(attr.into(), item.into()).into()
}
