use secret_manager_macro::SecretManager;

// TODO should go and check that a secret with this name exists
//  should load them (later maybe do it once - but also option to refresh...)
//  should keep the existing attributes
#[SecretManager]
struct SampleSecrets {}

fn main() {}
