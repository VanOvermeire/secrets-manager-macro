## Setup

In addition to this crate, you will need the following dependencies:

```toml
[dependencies]
aws-config = "0.54.1"
aws-sdk-secretsmanager = "0.24.0"
serde_json = "1.0.93"
tokio = { version = "1.26.0", features = ["full"] }
```

And if you are running inside an AWS Lambda, you should already have most of these.

## Usage

### Simple usage (no environment specific secrets)

```rust
use secrets_manager_macro::build_secrets_struct;

#[tokio::main]
async fn main() {
    #[derive(Debug)] // (you can have other annotations in addition to build_secrets_struct)
    #[build_secrets_struct]
    struct NoPrefixSecret {}

    let secrets = NoPrefixSecret::new().await;

    // secrets are properties of the struct, so you can now access them
    assert_eq!(secrets.thirdKey.as_ref(), "thirdValue");
}
```

During compilation, the macro will check that this secret exist. It also accepts several other case styles, so `ExampleSecret`, `example-secret` or `example_secret`
are all acceptable. If no valid secret is found, or if you have no active AWS credentials, compile-time errors will be thrown.

Next, it will use the value in your secret to add fields to the `ExampleSecret` struct. In the above example, `firstKey` is one of those
secrets. The macro expects JSON as the secret value. If this is not the case, another compile-time error will be thrown.

The values are obviously not added to your code, that would be dangerous. They only used at runtime, when calling `new()`. 
At that point, the generated code will look for a secret with the name that was found during compilation (`ExampleSecret`, `example-secret` or `example_secret`).
The values of the secret can now be accessed via their names.

`new` will panic if the secret is not present, values are missing or if the secret value is not valid JSON.
This seems acceptable behavior since. We already checked most of this information at compile time, so the chance our code will panic is small.
And we cannot really continue running most applications without secret, so it is better to stop the application dead in its tracks.

### Usage with environment specific secrets

```rust
use secrets_manager_macro::build_secrets_struct;

#[tokio::main]
async fn main() {
    std::env::set_var("ENV", "dev");

    #[build_secrets_struct(envs = dev,prod)]
    struct SecretsManagerTestSecret {}

    let secrets = SecretsManagerTestSecret::new().await;

    // secrets are properties of the struct, so you can now access them
    assert_eq!(secrets.firstKey.as_ref(), "firstValue");
}
```

Similar to the above, 'simple', setup,the macro will look for matches (`ExampleSecret`, `example-secret` or `example_secret`) but this
time they have to be prefixed with the passed in `envs`. So, for example, `/dev/ExampleSecret` would be a match.
Again, compilation will fail if there are no credentials or if the secret is missing for one of the specified envs.

Next, it will use the `dev` secret values to add fields to the `ExampleSecret` struct. (So you currently need to have dev as an env. This limitation will disappear in time.)
In the above example, `firstKey` is one of those secrets. Like before, the macro expects JSON as the secret value.

The values are only used at runtime, when calling `new()`. At that point, the generated code will look for a secret with the name that
was found during compilation (`ExampleSecret`, `example-secret` or `example_secret`), prefixed with the contents of the `ENV` _or_ `ENVIRONMENT` environment variable.
In the above case, assuming the found secret was called `example_secret`, the code will look for `/dev/example_secret`. 

As before, `new` will panic if the secret is not present, values are missing or if the secret value is not valid JSON.
