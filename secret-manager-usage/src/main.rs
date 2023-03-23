use secret_manager_macro::build_secrets_struct;

build_secrets_struct!(secret_name: "SampleSecrets");

// TODO compilation tests

#[tokio::main]
async fn main() {
    let secrets = Secrets::new().await;
    println!("{:?}", secrets);
}
