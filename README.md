# Secret Manager Macro

A macro for using AWS Secret Manager secrets in your application.

## Status

![Github Actions Status](https://github.com/VanOvermeire/secret-manager-macro/actions/workflows/github-deploy.yml/badge.svg)

## Features

- Compile time checks
- Automatic loading of secrets into the chosen struct
- Sensible defaults where possible

## Usage

In addition to this crate, you will need the following dependencies:

```toml
[dependencies]
aws-config = "0.54.1"
aws-sdk-secretsmanager = "0.24.0"
serde_json = "1.0.93"
tokio = { version = "1.26.0", features = ["full"] }
```

And if you are running inside an AWS Lambda, you should already have most of these.

Example code:

```rust
#[tokio::main]
async fn main() {
    std::env::set_var("ENV", "dev");

    #[build_secrets_struct(envs = dev,prod)]
    struct ExampleSecret {}

    let secrets = ExampleSecret::new().await;

    // secrets are properties of the struct, so you can now access them
    assert_eq!(secrets.firstKey.as_ref(), "firstValue");
}
```

During compilation, the macro will check that secrets exist for the provided `envs`. In this case it will make sure
that `/dev/ExampleSecret` and `/prod/ExampleSecret` exist. Instead of `ExampleSecret`, `example-secret` or `example_secret` are
also considered valid. If no valid secret is found, or if you have no active AWS credentials, compile-time errors will be thrown.

Next, it will use the `dev` secret values to add fields to the `ExampleSecret` struct. In the above example, `firstKey` is one of those
secrets. The macro expects JSON as the secret value. If this is not the case, another compile-time error will be thrown.

The values are only used at runtime, when calling `new()`. At that point, the generated code will look for a secret with the name that
was found during compilation (`ExampleSecret`, `example-secret` or `example_secret`), prefixed with the contents of the `ENV` variable.
In the above case, the prefix is `/dev/`. The values of the secret now be added. 

`new` will panic if the secret is not present, values are missing, the secret value is not valid JSON... 
This is acceptable since: 
- we already checked most of this information at compile time
- we cannot really continue running most applications without secrets

See the `tests` folder for additional examples.

// TODO examples without env!

## Running the tests

The integration tests require that you have valid AWS credentials and an AWS Secret Manager secret called 'GithubActionsTestSecret' in your AWS account.
You can use `secret_manager_tests_setup.sh` in `scripts` to create this secret with the correct secret value. 
`secret_manager_delete_tests_setup.sh` will clean up these test secrets. 

At the time of writing, a secret will cost you 40 dollar cents per month, plus 5 cents for 10 000 API calls.

## TODOs

- Expand documentation
- GitHub actions publish + end-to-end test with deploy lambda and check output

- Accept ENVIRONMENT as an alternative to ENV (any other vars?)
- Only allow the passed in `envs` when calling `new`?
- Check all env contents
- Attribute for changing secret name
- Attribute for checking **selection** of envs + allow disabling of compile time checks (more useful once you can add fields to the struct...)? Or 'saving' of a check?
- Handling nextToken for list secrets
- Refresh method (and keep a validity timestamp)
- Lazy static option?
- Allow selection of secrets by adding fields to the struct
- Parameter store as an alternative for loading? As an additional macro?