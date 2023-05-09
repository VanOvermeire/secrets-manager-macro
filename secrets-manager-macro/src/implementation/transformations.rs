use std::cmp::Ordering;
use std::collections::HashMap;

use proc_macro2::{Ident, Span};

use crate::implementation::aws::NonEmptySecrets;
use crate::implementation::errors::RetrievalError;
use crate::implementation::input::EnvSetting;

const HYPHEN: char = '-';
const UNDERSCORE: char = '_';

pub struct ValidatedSecrets {
    secrets: Vec<String>,
    env_setting: EnvSetting,
}

impl ValidatedSecrets {
    pub fn new(found_secret_names: NonEmptySecrets, env_setting: EnvSetting) -> Result<Self, RetrievalError> {
        match &env_setting {
            EnvSetting::None if found_secret_names.0.len() != 1 => Err(RetrievalError::DuplicateSecrets(format!("expected to find an exact match, instead found multiple possible secrets: {}. Please specify an exact name", found_secret_names.0.join(",")))),
            EnvSetting::None => Ok(ValidatedSecrets {
                secrets: found_secret_names.0,
                env_setting,
            }),
            EnvSetting::Env(envs) => {
                let matched: Vec<String> = found_secret_names.0.clone().into_iter().filter(|s| envs.iter().any(|e| s.contains(e))).collect();

                match matched.len().cmp(&envs.len()) {
                    Ordering::Equal => Ok(ValidatedSecrets {
                        secrets: matched,
                        env_setting,
                    }),
                    Ordering::Less => Err(RetrievalError::MissingEnv(format!("received envs {} but only matched these secrets: {}", envs.join(","), matched.join(",")))),
                    Ordering::Greater => Err(RetrievalError::DuplicateSecrets(format!("expected to find {} secrets, but found more: {}. Please specify an exact name", envs.len(), found_secret_names.0.join(",")))),
                }
            }
        }
    }

    // would be nice to also support suffix (secret-something/dev)? but would also need to *know* where this is for generating the right 'get' call in output
    pub fn get_full_and_base_secret(&self) -> (String, String) {
        match &self.env_setting {
            EnvSetting::None => {
                // validated during creation: we have *one* match. Panic should not occur
                let full = self.secrets.first()
                    .unwrap_or_else(|| self.secrets.first().expect("Found secrets to contain at least one secret"))
                    .to_string();
                // there is no prefix, so base and full are identical
                (full.clone(), full)
            }
            EnvSetting::Env(envs) => {
                // TODO this assumes that you passed in a dev env. Perhaps better to check all secrets?
                //  alternatively, pick one secret and assume they all have the same fields
                let full = self.secrets.iter()
                    .find(|s| s.contains("/dev/"))
                    .unwrap_or_else(|| self.secrets.first().expect("Found secrets to contain at least one secret"))
                    .to_string();
                let base = envs.iter().fold(full.clone(), |acc, curr| {
                    acc.replace(&format!("/{curr}/"), "")
                });

                (full, base)
            }
        }
    }
}

pub fn possible_base_names(secret_struct_name: &str) -> Vec<String> {
    let with_hyphen = lowercase_and_add(secret_struct_name, |mut acc| {
        acc.push(HYPHEN);
        acc
    });
    let with_underscore = lowercase_and_add(secret_struct_name, |mut acc| {
        acc.push(UNDERSCORE);
        acc
    });
    vec![secret_struct_name.to_string(), with_hyphen, with_underscore]
        .into_iter()
        .fold(vec![], |mut acc, curr| {
            if !acc.contains(&curr) {
                acc.push(curr);
            }
            acc
        })
}

fn lowercase_and_add<F>(secret_struct_name: &str, mut addition: F) -> String where F: FnMut(String) -> String {
    secret_struct_name.chars().fold("".to_string(), |mut acc, curr| {
        if acc.is_empty() {
            acc.push_str(&curr.to_lowercase().to_string());
            acc
        } else if curr.is_uppercase() {
            let mut acc = addition(acc);
            acc.push_str(&curr.to_lowercase().to_string());
            acc
        } else {
            acc.push(curr);
            acc
        }
    })
}

pub fn keys_as_ident_list(key_map: HashMap<String, String>) -> Vec<Ident> {
    key_map
        .keys()
        .map(|k| Ident::new(k, Span::call_site()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_should_work_when_no_envs_are_present_and_one_secret() {
        let found_secrets = NonEmptySecrets(vec!["sample-secret".to_string()]);
        let env = EnvSetting::None;

        let actual = ValidatedSecrets::new(found_secrets, env);

        assert!(actual.is_ok());
        assert_eq!(actual.unwrap().secrets.len(), 1);
    }

    #[test]
    fn validate_should_fail_when_no_envs_are_present_and_multiple_secrets() {
        let found_secrets = NonEmptySecrets(vec!["sample-secret".to_string(), "fake-secret".to_string()]);
        let env = EnvSetting::None;

        let actual = ValidatedSecrets::new(found_secrets, env);

        assert!(actual.is_err());
    }

    #[test]
    fn validate_should_work_when_all_envs_are_present_filtering_out_unknowns() {
        let found_secrets = NonEmptySecrets(vec!["/prod/sample-secret".to_string(), "/dev/sample-secret".to_string(), "/fake/sample-secret".to_string()]);
        let envs = EnvSetting::Env(vec!["dev".to_string(), "prod".to_string()]);

        let actual = ValidatedSecrets::new(found_secrets, envs);

        assert!(actual.is_ok());
        assert_eq!(actual.unwrap().secrets.len(), 2);
    }

    #[test]
    fn validate_should_fail_when_too_many_envs_are_present() {
        let found_secrets = NonEmptySecrets(vec!["/prod/sample-secret".to_string(), "/dev/sample-secret".to_string(), "/prod/SampleSecret".to_string(), "/fake/sample-secret".to_string()]);
        let envs = EnvSetting::Env(vec!["dev".to_string(), "prod".to_string()]);

        let actual = ValidatedSecrets::new(found_secrets, envs);

        assert!(actual.is_err());
    }

    #[test]
    fn validate_should_fail_when_not_all_envs_are_present() {
        let found_secrets = NonEmptySecrets(vec!["/prod/sample-secret".to_string()]);
        let env = EnvSetting::Env(vec!["dev".to_string(), "prod".to_string()]);

        let actual = ValidatedSecrets::new(found_secrets, env);

        assert!(actual.is_err());
    }

    #[test]
    fn get_full_and_base_secret_should_fail_when_no_multiple_matches_for_none_env() {
        let found_secrets = NonEmptySecrets(vec!["sample-secret".to_string(), "OtherSecret".to_string()]);
        let env = EnvSetting::None;

        let actual = ValidatedSecrets::new(found_secrets, env);

        assert!(actual.is_err());
    }

    #[test]
    fn get_full_and_base_secret_should_get_exact_match_and_identical_base_and_full() {
        let found_secrets = NonEmptySecrets(vec!["sample-secret".to_string()]);
        let env = EnvSetting::None;

        let actual = ValidatedSecrets::new(found_secrets, env).unwrap();
        let (actual_full, actual_base) = actual.get_full_and_base_secret();

        assert_eq!(actual_full, "sample-secret");
        assert_eq!(actual_base, "sample-secret");
    }

    #[test]
    fn get_full_and_base_secret_should_get_an_env_when_dev_is_not_available() {
        let found_secrets = NonEmptySecrets(vec!["/prod/sample-secret".to_string(), "/acc/sample-secret".to_string()]);
        let env = EnvSetting::Env(vec!["prod".to_string(), "acc".to_string()]);

        let actual = ValidatedSecrets::new(found_secrets, env).unwrap();
        let (actual_full, actual_base) = actual.get_full_and_base_secret();

        assert_eq!(actual_full, "/prod/sample-secret");
        assert_eq!(actual_base, "sample-secret");
    }

    #[test]
    fn get_full_and_base_secret_should_by_fallback_to_first_secret() {
        let found_secrets = NonEmptySecrets(vec!["sample-secret".to_string()]);
        let env = EnvSetting::None;

        let actual = ValidatedSecrets::new(found_secrets, env).unwrap();
        let (actual_full, actual_base) = actual.get_full_and_base_secret();

        assert_eq!(actual_full, "sample-secret");
        assert_eq!(actual_base, "sample-secret");
    }

    #[test]
    fn possible_base_names_should_create_alternative_secret_struct_names_and_add_to_the_original() {
        let actual = possible_base_names("ExampleSecret");

        assert_eq!(actual.len(), 3);
        assert_eq!(actual[0], "ExampleSecret");
        assert_eq!(actual[1], "example-secret");
        assert_eq!(actual[2], "example_secret");
    }

    #[test]
    fn possible_base_names_should_create_alternative_secret_struct_names_and_filter_out_duplicates() {
        let actual = possible_base_names("Secret");

        assert_eq!(actual.len(), 2);
        assert_eq!(actual[0], "Secret");
        assert_eq!(actual[1], "secret");
    }

    #[test]
    fn lowercase_and_add_should_just_lowercase_name_when_addition_adds_nothing() {
        let actual = lowercase_and_add("SecretName", |val| val);

        assert_eq!(actual, "secretname".to_string());
    }

    #[test]
    fn lowercase_and_add_should_lowercase_and_add_given_value_before_capital_letter_except_first() {
        let actual = lowercase_and_add("SecretSecretName", |mut val| {
            val.push('*');
            val
        });

        assert_eq!(actual, "secret*secret*name".to_string());
    }

    #[test]
    fn keys_as_ident_list_should_create_idents_from_keys_and_ignore_values() {
        let mut keys = HashMap::new();
        keys.insert("firstKey".to_string(), "firstValue".to_string());
        keys.insert("secondKey".to_string(), "secondValue".to_string());

        let actual = keys_as_ident_list(keys);

        let as_strings: Vec<String> = actual.iter().map(|v| v.to_string()).collect();

        assert_eq!(actual.len(), 2);
        assert!(as_strings.contains(&"firstKey".to_string()));
        assert!(as_strings.contains(&"secondKey".to_string()));
    }
}