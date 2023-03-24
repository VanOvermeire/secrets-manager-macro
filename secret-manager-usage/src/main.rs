// use secret_manager_macro::build_secrets_struct;
use secret_manager_macro::build_secrets_structz;

// build_secrets_struct!(secret_name: "SampleSecrets");

// TODO compilation tests

#[derive(Debug)]
#[build_secrets_structz]
struct Secrets {}

// #[build_secrets_structz]
// struct OtherSecret {}

#[tokio::main]
async fn main() {
    // let secrets = Secrets::new().await;
    // println!("{:?}", secrets);
}
