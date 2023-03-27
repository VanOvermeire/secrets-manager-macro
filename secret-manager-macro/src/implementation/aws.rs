use std::collections::HashMap;

use aws_sdk_secretsmanager::output::{GetSecretValueOutput, ListSecretsOutput};

use crate::implementation::errors::RetrievalError;
use crate::implementation::transformations::ValidatedSecrets;
use aws_sdk_secretsmanager::Client;
use aws_sdk_secretsmanager::error::{GetSecretValueError, ListSecretsError};
use aws_sdk_secretsmanager::types::SdkError;

struct SecretManagerClient {
    client: Client
}

impl SecretManagerClient {
    async fn new() -> Self {
        let shared_config = aws_config::from_env().load().await;
        SecretManagerClient {
            client: Client::new(&shared_config),
        }
    }

    async fn list_secrets(&self) -> Result<ListSecretsOutput, SdkError<ListSecretsError>> {
        self.client.list_secrets().send().await
    }

    async fn get_filtered_secret_list(&self, base_secret_names: Vec<String>) -> Result<Vec<String>, RetrievalError> {
        let list_result = self.list_secrets().await?;
        Ok(filter_secrets_list(list_result, base_secret_names)?)
    }

    async fn get_secret(&self, secret_name: &str) -> Result<GetSecretValueOutput, SdkError<GetSecretValueError>> {
        self.client
            .get_secret_value()
            .secret_id(secret_name)
            .send()
            .await
    }

    async fn get_secret_as_map(&self, full_secret_name: &str) -> Result<HashMap<String, String>, RetrievalError> {
        let secret_value = self.get_secret(&full_secret_name).await?;
        get_secret_value_as_map(secret_value)
    }
}

fn get_secret_value_as_map(output: GetSecretValueOutput) -> Result<HashMap<String, String>, RetrievalError> {
    let content = output
        .secret_string()
        .map_or_else(|| "{}".to_string(), |v| v.to_string());
    Ok(serde_json::from_str(&content)?)
}

fn filter_secrets_list(output: ListSecretsOutput, base_secret_names: Vec<String>) -> Result<Vec<String>, RetrievalError> {
    let possible_secrets: Vec<String> = output
        .secret_list()
        .ok_or_else(|| RetrievalError::NotFound("No secrets found in AWS account".to_string()))?
        .iter()
        .filter_map(|v| v.name())
        .map(|v| v.to_string())
        .filter(|v| {
            // exact match *or* at least with a forward slash in front
            base_secret_names.contains(v) ||
                base_secret_names.iter().any(|name| v.contains(&format!("/{}", name)))
        }).collect();

    if possible_secrets.is_empty() {
        Err(
            RetrievalError::NotFound(format!(
                "Could not find secret with any of these names: {}",
                base_secret_names.join(",")
            ))
        )
    } else {
        Ok(possible_secrets)
    }
}

pub async fn call_secret_manager(base_secret_names: Vec<String>, envs: Vec<String>) -> Result<(String, HashMap<String, String>), RetrievalError> {
    let client = SecretManagerClient::new().await;
    let found_secret_names = client.get_filtered_secret_list(base_secret_names).await?;

    let validated_secrets = ValidatedSecrets::new(found_secret_names, envs)?;
    let (full_secret_name, actual_base_name) = validated_secrets.get_full_and_base_secret();

    let secret_value = client.get_secret_as_map(&full_secret_name).await?;
    Ok((actual_base_name, secret_value))
}

#[cfg(test)]
mod tests {
    use aws_sdk_secretsmanager::model::SecretListEntry;

    use super::*;

    #[test]
    fn filter_secrets_list_should_find_secret_with_given_name() {
        let list = create_secret_list();
        let possible_names = vec!["SampleSecret".to_string(), "sample-secret".to_string(), "sample_secret".to_string()];

        let actual = filter_secrets_list(list, possible_names).unwrap();

        assert_eq!(actual, vec!["/prod/sample-secret"]);
    }

    #[test]
    fn filter_secrets_list_should_find_secret_ignoring_secret_that_looks_similar() {
        let list = ListSecretsOutput::builder()
            .secret_list(SecretListEntry::builder().name("AnotherKindOfSampleSecret").build())
            .secret_list(SecretListEntry::builder().name("/prod/sample-secret").build())
            .build();
        let possible_names = vec!["SampleSecret".to_string(), "sample-secret".to_string(), "sample_secret".to_string()];

        let actual = filter_secrets_list(list, possible_names).unwrap();

        assert_eq!(actual, vec!["/prod/sample-secret"]);
    }

    #[test]
    fn filter_secrets_list_should_return_error_for_unknown_secret() {
        let list = create_secret_list();

        let actual = filter_secrets_list(list, vec!["Unknown".to_string()]);

        assert!(actual.is_err());
    }

    #[test]
    fn filter_secrets_list_should_return_error_when_there_are_no_secrets() {
        let list = ListSecretsOutput::builder().build();

        let actual = filter_secrets_list(list, vec!["DoesNotMatter".to_string()]);

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
            .secret_list(SecretListEntry::builder().name("/dev/fake-secret").build())
            .secret_list(SecretListEntry::builder().name("FakeSecret").build())
            .secret_list(SecretListEntry::builder().name("/prod/sample-secret").build())
            .build();
        list
    }
}
