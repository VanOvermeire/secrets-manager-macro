use lambda_runtime::{Error, LambdaEvent, service_fn};
use serde_json::Value;
use serde_json::json;
use secrets_manager_macro::build_secrets_struct;

#[build_secrets_struct(envs = dev,prod)]
struct SecretsManagerTestSecret {}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(move |_: LambdaEvent<Value>| {
        flow()
    })).await
}

async fn flow() -> Result<Value, Error> {
    let env = std::env::var("ENV").unwrap_or_else(|_| "unknown".to_string());
    println!("Invoked test lambda for environment {}", env);

    let secrets = SecretsManagerTestSecret::new().await;

    Ok(json!({
        "firstValue": secrets.firstKey.as_ref(),
        "secondValue": secrets.secondKey.as_ref(),
    }))
}
