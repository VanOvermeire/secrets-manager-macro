use std::collections::HashMap;

use aws_sdk_secretsmanager::Client;
use aws_sdk_secretsmanager::error::{GetSecretValueError, ListSecretsError};
use aws_sdk_secretsmanager::output::{GetSecretValueOutput, ListSecretsOutput};
use aws_sdk_secretsmanager::types::SdkError;

use crate::implementation::errors::RetrievalError;
use crate::implementation::errors::RetrievalError::MissingEnv;

async fn build_client() -> Client {
    let shared_config = aws_config::from_env().load().await;
    Client::new(&shared_config)
}

async fn list_secrets(client: &Client) -> Result<ListSecretsOutput, SdkError<ListSecretsError>> {
    client.list_secrets().send().await
}

async fn get_secret(client: &Client, secret_name: &str) -> Result<GetSecretValueOutput, SdkError<GetSecretValueError>> {
    client
        .get_secret_value()
        .secret_id(secret_name)
        .send()
        .await
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

// would be nice to also support suffix (secret-something/dev)? but would also need to *know* where this is for generating the right 'get' call in output
fn get_full_and_base_secret(found_secret_names: &Vec<String>, envs: &Vec<String>) -> (String, String) {
    if envs.is_empty() {
        let full = found_secret_names.iter()
            .find(|s| s.contains("/dev/"))
            .unwrap_or_else(|| found_secret_names.first().expect("Found secrets to contain at least one secret"))
            .to_string();
        let base = full.replace("/dev/", "");

        (full, base)
    } else {
        let full = found_secret_names.iter()
            .find(|s| s.contains("/dev/"))
            .unwrap_or_else(|| found_secret_names.first().expect("Found secrets to contain at least one secret"))
            .to_string();
        let base = envs.iter().fold(full.clone(), |acc, curr| {
            acc.replace(&format!("/{}/", curr), "")
        });

        (full, base)
    }
}

// TODO return ValidatedSecrets - and use that in the next methods - and move to separate package
fn validate_secrets(found_secret_names: Vec<String>, envs: &Vec<String>) -> Result<Vec<String>, RetrievalError> {
    if envs.is_empty() {
        Ok(found_secret_names)
    } else {
        let matched: Vec<String> = found_secret_names.into_iter().filter(|s| envs.iter().any(|e| s.contains(e))).collect();

        if matched.len() == envs.len() {
            Ok(matched)
        } else {
            Err(MissingEnv(format!("Received these envs {} but only matched these secrets {}", matched.join(","), envs.join(","))))
        }
    }
}

async fn call_secret_manager(base_secret_names: Vec<String>, envs: &Vec<String>) -> Result<(String, HashMap<String, String>), RetrievalError> {
    let client = build_client().await;

    let list_result = list_secrets(&client).await?;
    let found_secret_names = filter_secrets_list(list_result, base_secret_names)?;
    let matched_secrets = validate_secrets(found_secret_names, envs)?;
    let (full_secret_name, actual_base_name) = get_full_and_base_secret(&matched_secrets, &envs);

    let secret_value = get_secret(&client, &full_secret_name).await?;
    get_secret_value_as_map(secret_value).map(|v| (actual_base_name, v))
}

pub fn retrieve_real_name_and_keys(base_secret_names: Vec<String>, envs: Vec<String>) -> Result<(String, HashMap<String, String>), RetrievalError> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(call_secret_manager(base_secret_names, &envs))
}

#[cfg(test)]
mod tests {
    use aws_sdk_secretsmanager::model::SecretListEntry;

    use super::*;

    #[test]
    fn validate_should_work_when_all_envs_are_present_filtering_out_unknowns() {
        let found_secrets = vec!["/prod/sample-secret".to_string(), "/dev/sample-secret".to_string(), "/fake/sample-secret".to_string()];
        let envs = vec!["dev".to_string(), "prod".to_string()];

        let actual = validate_secrets(found_secrets, &envs);

        assert!(actual.is_ok());
        assert_eq!(actual.unwrap().len(), 2);
    }

    #[test]
    fn validate_should_work_when_no_envs_are_present() {
        let found_secrets = vec!["/prod/sample-secret".to_string(), "/dev/sample-secret".to_string()];
        let envs = vec![];

        let actual = validate_secrets(found_secrets, &envs);

        assert!(actual.is_ok());
        assert_eq!(actual.unwrap().len(), 2);
    }

    #[test]
    fn validate_should_fail_when_not_all_envs_are_present() {
        let found_secrets = vec!["/prod/sample-secret".to_string()];
        let envs = vec!["dev".to_string(), "prod".to_string()];

        let actual = validate_secrets(found_secrets, &envs);

        assert!(actual.is_err());
    }

    #[test]
    fn get_full_and_base_secret_should_by_default_prefer_dev() {
        let found_secrets = vec!["/prod/sample-secret".to_string(), "/dev/sample-secret".to_string()];

        let (actual_full, actual_base) = get_full_and_base_secret(&found_secrets, &vec![]);

        assert_eq!(actual_full, "/dev/sample-secret");
        assert_eq!(actual_base, "sample-secret");
    }

    #[test]
    fn get_full_and_base_secret_should_get_an_env_when_dev_is_not_available() {
        let found_secrets = vec!["/prod/sample-secret".to_string(), "/acc/sample-secret".to_string()];

        let (actual_full, actual_base) = get_full_and_base_secret(&found_secrets, &vec!["prod".to_string(), "acc".to_string()]);

        assert_eq!(actual_full, "/prod/sample-secret");
        assert_eq!(actual_base, "sample-secret");
    }

    #[test]
    fn get_full_and_base_secret_should_by_fallback_to_first_secret() {
        let found_secrets = vec!["sample-secret".to_string()];

        let (actual_full, actual_base) = get_full_and_base_secret(&found_secrets, &vec![]);

        assert_eq!(actual_full, "sample-secret");
        assert_eq!(actual_base, "sample-secret");
    }

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
