use secrets_manager_macro::build_secrets_struct;

#[build_secrets_struct(envs = dev,fake)]
struct SecretManagerTestSecret {}

fn main() {}
