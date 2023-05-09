use secrets_manager_macro::build_secrets_struct;

#[tokio::test]
async fn should_retrieve_secrets_with_specified_envs_and_retrieve_correct_values() {
    std::env::set_var("ENV", "prod");

    #[build_secrets_struct(envs = dev,prod)]
    struct SecretsManagerTestSecret {}

    let secrets = SecretsManagerTestSecret::new().await;

    assert_eq!(secrets.firstKey.as_ref(), "prodValue");
    assert_eq!(secrets.secondKey.as_ref(), "secondProdValue");
}
