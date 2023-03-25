# Secret Manager Macro

A macro for using AWS Secret Manager secrets in your application.

## Status

![Github Actions Status](https://github.com/VanOvermeire/secret-manager-macro/actions/workflows/github-deploy.yml/badge.svg)

## Features

- Compile time checks
- Little config required

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

Example code. This assumes you have a secret called 'Secrets' in your AWS environment. 
It will throw an error if this is not the case, or if you have no valid credentials.

```rust
#[tokio::main]
async fn main() {
    #[derive(Debug)]
    #[build_secrets_struct]
    struct Secrets {}

    let secrets = Secrets::new().await;

    // secrets are properties of the struct, so you can now access them
    assert_eq!(secrets.firstKey.as_ref(), "firstValue");
}
```

## Running the tests

The integration tests require that you have valid AWS credentials and an AWS Secret Manager secret called 'Secrets' in your AWS account.
At the time of writing, a secret will cost you 40 dollar cents per month, plus 5 cents for 10 000 API calls.

## TODOs

- Add script for setup of env (creation of secrets)
- Expand the documentation
- GitHub actions improvements
- Parameter store as an alternative for loading? As an additional macro?
- Test for JSON error

- Handling nextToken for list secrets
- Search for similar names
- Allow for different envs (look for env var?)
- Refresh method
- Lazy static option?
- Allow selection of secrets by adding fields to the struct
- Allow disabling of compile time checks (more useful once you can add fields to the struct...)? Or 'saving' of a check?
