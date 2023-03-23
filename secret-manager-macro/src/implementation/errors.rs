use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use aws_sdk_secretsmanager::error::{GetSecretValueError, ListSecretsError};
use aws_sdk_secretsmanager::types::SdkError;

#[derive(Debug)]
pub enum RetrievalError {
    AWSError(String),
    NotFoundError(String),
    JSONError,
}

impl Display for RetrievalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Retrieval Error")
    }
}

impl From<serde_json::Error> for RetrievalError {
    fn from(_: serde_json::Error) -> Self {
        RetrievalError::JSONError
    }
}

impl From<SdkError<ListSecretsError>> for RetrievalError {
    fn from(value: SdkError<ListSecretsError>) -> Self {
        RetrievalError::AWSError(value.to_string())
    }
}

impl From<SdkError<GetSecretValueError>> for RetrievalError {
    fn from(value: SdkError<GetSecretValueError>) -> Self {
        RetrievalError::AWSError(value.to_string())
    }
}

impl Error for RetrievalError {}
