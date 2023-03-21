use proc_macro::TokenStream;
use secret_manager_code::create_secret_manager;

#[proc_macro_attribute]
pub fn SecretManager(_attr: TokenStream, item: TokenStream) -> TokenStream {
    create_secret_manager(item.into()).into()
}
