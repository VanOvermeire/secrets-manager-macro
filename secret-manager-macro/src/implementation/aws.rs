use crate::implementation::errors::RetrievalError;
use aws_sdk_secretsmanager::error::{GetSecretValueError, ListSecretsError};
use aws_sdk_secretsmanager::output::{GetSecretValueOutput, ListSecretsOutput};
use aws_sdk_secretsmanager::types::SdkError;
use aws_sdk_secretsmanager::Client;
use std::collections::HashMap;

async fn build_client() -> Client {
    let shared_config = aws_config::from_env().load().await;
    Client::new(&shared_config)
}

// TODO handle next token
async fn list_secrets(client: &Client) -> Result<ListSecretsOutput, SdkError<ListSecretsError>> {
    client.list_secrets().send().await
}

async fn get_secret(
    client: &Client,
    secret_name: &str,
) -> Result<GetSecretValueOutput, SdkError<GetSecretValueError>> {
    client
        .get_secret_value()
        .secret_id(secret_name)
        .send()
        .await
}

fn get_secret_value_as_map(
    output: GetSecretValueOutput,
) -> Result<HashMap<String, String>, RetrievalError> {
    let content = output
        .secret_string()
        .map_or_else(|| "{}".to_string(), |v| v.to_string());
    Ok(serde_json::from_str(&content)?)
}

// not really needed yet but we will offer multiple options later
fn filter_secrets_list(
    output: ListSecretsOutput,
    secret_name: &str,
) -> Result<String, RetrievalError> {
    output
        .secret_list()
        .ok_or_else(|| RetrievalError::NotFoundError("No secrets found in AWS account".to_string()))?
        .iter()
        .filter_map(|v| v.name())
        .map(|v| v.to_string())
        .find(|v| v.eq(&secret_name))
        .map(|v| v.to_string())
        .ok_or_else(|| {
            RetrievalError::NotFoundError(format!(
                "Could not find secret with name {}",
                secret_name
            ))
        })
}

async fn call_secret_manager(
    secret_name: &str,
) -> Result<(String, HashMap<String, String>), RetrievalError> {
    let client = build_client().await;

    let result = list_secrets(&client).await?;
    let actual_secret_name = filter_secrets_list(result, secret_name)?;
    let secret_value = get_secret(&client, &actual_secret_name).await?;
    get_secret_value_as_map(secret_value).map(|v| (actual_secret_name, v))
}

pub fn retrieve_real_name_and_keys(
    secret_name: &str,
) -> Result<(String, HashMap<String, String>), RetrievalError> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(call_secret_manager(secret_name))
}

#[cfg(test)]
mod tests {
    use aws_sdk_secretsmanager::model::SecretListEntry;

    use super::*;

    #[test]
    fn filter_secrets_list_should_find_secret_with_given_name() {
        let list = create_secret_list();

        let actual = filter_secrets_list(list, "Secret").unwrap();

        assert_eq!(actual, "Secret");
    }

    #[test]
    fn filter_secrets_list_should_return_error_for_unknown_secret() {
        let list = create_secret_list();

        let actual = filter_secrets_list(list, "Unknown");

        assert!(actual.is_err());
    }

    #[test]
    fn get_secret_value_should_return_secrets_as_hashmap() {
        let secret_value = r#"{
            "key1": "value1", "key2": "value2"
        }"#;
        let output = GetSecretValueOutput::builder()
            .secret_string(secret_value)
            .build();

        let actual = get_secret_value_as_map(output).unwrap();

        assert_eq!(actual.len(), 2);
        assert_eq!(actual.get("key1").unwrap(), &"value1");
        assert_eq!(actual.get("key2").unwrap(), &"value2");
    }

    #[test]
    fn get_secret_value_should_return_empty_hashmap_when_no_secret_is_present() {
        let output = GetSecretValueOutput::builder().build();

        let actual = get_secret_value_as_map(output).unwrap();

        assert_eq!(actual.len(), 0);
    }

    #[test]
    fn get_secret_value_should_return_error_if_parsing_fails() {
        let secret_value = "{ invalid }";
        let output = GetSecretValueOutput::builder()
            .secret_string(secret_value)
            .build();

        let actual = get_secret_value_as_map(output);

        assert!(actual.is_err());
    }

    fn create_secret_list() -> ListSecretsOutput {
        let list = ListSecretsOutput::builder()
            .secret_list(SecretListEntry::builder().name("Secret").build())
            .secret_list(SecretListEntry::builder().name("NotSecret").build())
            .build();
        list
    }
}
