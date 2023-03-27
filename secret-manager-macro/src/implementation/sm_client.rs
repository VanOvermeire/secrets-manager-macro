use aws_sdk_secretsmanager::Client;
use aws_sdk_secretsmanager::error::{GetSecretValueError, ListSecretsError};
use aws_sdk_secretsmanager::output::{GetSecretValueOutput, ListSecretsOutput};
use aws_sdk_secretsmanager::types::SdkError;

pub struct SecretManagerClient {
    client: Client
}

impl SecretManagerClient {
    pub async fn new() -> Self {
        let shared_config = aws_config::from_env().load().await;
        SecretManagerClient {
            client: Client::new(&shared_config),
        }
    }

    pub async fn list_secrets(&self) -> Result<ListSecretsOutput, SdkError<ListSecretsError>> {
        self.client.list_secrets().send().await
    }

    pub async fn get_secret(&self, secret_name: &str) -> Result<GetSecretValueOutput, SdkError<GetSecretValueError>> {
        self.client
            .get_secret_value()
            .secret_id(secret_name)
            .send()
            .await
    }
}
