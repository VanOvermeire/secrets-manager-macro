
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use aws_sdk_secretsmanager::error::{GetSecretValueError, ListSecretsError};
use aws_sdk_secretsmanager::types::SdkError;
use proc_macro2::{Span, TokenStream};

#[derive(Debug)]
pub enum RetrievalError {
    Aws(String),
    NotFound(String),
    Json,
}

impl RetrievalError {
    pub fn into_compile_error(self, correct_span: Span) -> TokenStream {
        match self {
            RetrievalError::Aws(e) | RetrievalError::NotFound(e) => syn::Error::new(correct_span, e).into_compile_error(),
            RetrievalError::Json => syn::Error::new(correct_span, "Could not parse the secret value as JSON").into_compile_error(),
        }
    }
}

impl Display for RetrievalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Retrieval Error")
    }
}

impl From<serde_json::Error> for RetrievalError {
    fn from(_: serde_json::Error) -> Self {
        RetrievalError::Json
    }
}

impl From<SdkError<ListSecretsError>> for RetrievalError {
    fn from(value: SdkError<ListSecretsError>) -> Self {
        RetrievalError::Aws(value.to_string())
    }
}

impl From<SdkError<GetSecretValueError>> for RetrievalError {
    fn from(value: SdkError<GetSecretValueError>) -> Self {
        RetrievalError::Aws(value.to_string())
    }
}

impl Error for RetrievalError {}
