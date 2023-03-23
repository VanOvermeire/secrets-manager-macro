mod implementation;

use crate::implementation::entrypoint::create_secret_manager;
use proc_macro::TokenStream;

#[proc_macro]
pub fn build_secrets_struct(item: TokenStream) -> TokenStream {
    create_secret_manager(item.into()).into()
}
