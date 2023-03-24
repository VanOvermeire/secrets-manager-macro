use secret_manager_macro::build_secrets_struct;

// TODO compilation tests

#[derive(Debug)]
#[build_secrets_struct]
struct Secrets {}

#[tokio::main]
async fn main() {
    // let secrets = Secrets::new().await;
    // println!("{:?}", secrets);
}
