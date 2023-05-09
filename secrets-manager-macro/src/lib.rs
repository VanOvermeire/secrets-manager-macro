#![doc = include_str!("../README.md")]

mod implementation;

use crate::implementation::entrypoint;
use proc_macro::{TokenStream};

/// Will fill the given struct with secrets retrieved from AWS Secrets Manager
#[proc_macro_attribute]
pub fn build_secrets_struct(attr: TokenStream, item: TokenStream) -> TokenStream {
    entrypoint::create_secrets_manager(attr.into(), item.into()).into()
}
