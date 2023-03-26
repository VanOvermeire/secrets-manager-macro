use secret_manager_macro::build_secrets_struct;

#[tokio::test]
async fn should_retrieve_secrets() {
    std::env::set_var("ENV", "dev");

    #[derive(Debug)]
    #[build_secrets_struct]
    struct SecretManagerTestSecret {}

    let secrets = SecretManagerTestSecret::new().await;

    assert_eq!(secrets.firstKey.as_ref(), "firstValue");
    assert_eq!(secrets.secondKey.as_ref(), "secondValue");
}

// TODO test without prefix; test with envs
// TODO failing test with unknown env
//  failing test when no env present?
