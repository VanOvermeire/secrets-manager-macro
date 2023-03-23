use proc_macro2::TokenStream;
use quote::quote;

// there are libraries for this, but this way no additional import is needed (there are enough already)
pub fn create_secret_string_struct() -> TokenStream {
    quote! {
        pub struct SecretString(String);

        impl SecretString {
            pub fn new(input: String) -> Self {
                SecretString(input)
            }
        }

        impl std::fmt::Debug for SecretString {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&format!("{}****", self.0.chars().next().unwrap_or('*')))
            }
        }

        impl AsRef<str> for SecretString {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }
    }
}
