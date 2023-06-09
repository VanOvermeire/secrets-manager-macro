use assert_impl::assert_impl;
use secrets_manager_macro::build_secrets_struct;

#[tokio::test]
async fn should_retrieve_secrets_with_specified_envs() {
    std::env::set_var("ENV", "dev");

    #[build_secrets_struct(envs = dev,prod)]
    struct SecretsManagerTestSecret {}

    let secrets = SecretsManagerTestSecret::new().await;

    assert_eq!(secrets.firstKey.as_ref(), "firstValue");
    assert_eq!(secrets.secondKey.as_ref(), "secondValue");
}

#[tokio::test]
async fn should_retrieve_secrets_no_env_keeping_other_annotations() {
    #[derive(Debug)]
    #[build_secrets_struct]
    struct NoPrefixSecret {}

    let secrets = NoPrefixSecret::new().await;

    assert_eq!(secrets.thirdKey.as_ref(), "thirdValue");
    assert_impl!(core::fmt::Debug: NoPrefixSecret);
}
