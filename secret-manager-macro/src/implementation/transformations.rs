use std::collections::HashMap;
use proc_macro2::{Ident, Span};

const HYPHEN: char = '-';
const UNDERSCORE: char = '_';

pub fn possible_base_names(secret_struct_name: &str) -> Vec<String> {
    let with_hyphen = alternative_without_first_letter(secret_struct_name, |mut acc| {
        acc.push(HYPHEN);
        acc
    });
    let with_underscore = alternative_without_first_letter(secret_struct_name, |mut acc| {
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

fn alternative_without_first_letter<F>(secret_struct_name: &str, mut addition: F) -> String where F: FnMut(String) -> String {
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