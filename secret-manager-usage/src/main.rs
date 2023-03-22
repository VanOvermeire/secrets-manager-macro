use secret_manager_macro::build_secrets_struct;

// TODO macro that
//  - checks at compile time
//  - with as little config as possible (search for creds and region etc.)
//  - generates a struct with the secrets, based on the name
//  - loads from AWS and offers a refresh option
// TODO later:
//  - param store?
//  - lazy static?

build_secrets_struct!("SampleSecrets");

fn main() {}
