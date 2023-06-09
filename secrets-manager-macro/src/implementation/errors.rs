use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use aws_sdk_secretsmanager::error::{GetSecretValueError, ListSecretsError};
use aws_sdk_secretsmanager::types::SdkError;
use proc_macro2::{Span, TokenStream};

#[derive(Debug)]
pub enum RetrievalError {
    Aws(String),
    NotFound(String),
    MissingEnv(String),
    DuplicateSecrets(String),
    Json,
}

impl RetrievalError {
    pub fn into_compile_error(self, correct_span: Span) -> TokenStream {
        match self {
            RetrievalError::NotFound(e) | RetrievalError::DuplicateSecrets(e) => syn::Error::new(correct_span, e).into_compile_error(),
            RetrievalError::Json => syn::Error::new(correct_span, "could not parse the secret value as JSON").into_compile_error(),
            RetrievalError::MissingEnv(e) => syn::Error::new(correct_span, e).into_compile_error(),
            RetrievalError::Aws(e) => syn::Error::new(correct_span, e).into_compile_error(),
        }
    }
}

impl Display for RetrievalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("retrieval error")
    }
}

impl From<serde_json::Error> for RetrievalError {
    fn from(_: serde_json::Error) -> Self {
        RetrievalError::Json
    }
}

impl From<SdkError<ListSecretsError>> for RetrievalError {
    fn from(value: SdkError<ListSecretsError>) -> Self {
        match value {
            SdkError::ServiceError(v) => RetrievalError::Aws(format!(
                "could not list secrets {} - do you have valid AWS credentials?",
                v.err()
                    .message()
                    .map(|v| format!("({})", v))
                    .unwrap_or_else(|| "".to_string())
            )),
            _ => RetrievalError::Aws(format!("could not list secrets: {}", value)),
        }
    }
}

impl From<SdkError<GetSecretValueError>> for RetrievalError {
    fn from(value: SdkError<GetSecretValueError>) -> Self {
        RetrievalError::Aws(format!("could not get secret from AWS: {}", value))
    }
}

impl Error for RetrievalError {}
