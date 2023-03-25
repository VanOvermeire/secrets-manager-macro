use secret_manager_macro::build_secrets_struct;

#[tokio::test]
async fn should_retrieve_secrets() {
    #[derive(Debug)]
    #[build_secrets_struct]
    struct Secrets {}

    let secrets = Secrets::new().await;

    assert_eq!(secrets.firstKey.as_ref(), "firstValue");
    assert_eq!(secrets.secondKey.as_ref(), "secondValue");
}