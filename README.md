# Secret Manager Macro

A macro for using AWS Secret Manager secrets in your application.

## Features

- Compile time checks
- Little config required

## Usage

TODO!

Dependencies:

```toml

```

Example code:

```rust

```

## Running the tests

The integration tests require that you have valid AWS credentials and an AWS Secret Manager secret called 'Secrets' in your AWS account.
At the time of writing, a secret will cost you 40 dollar cents per month, plus 5 cents for 10 000 API calls.

## TODOs

- GitHub actions improvements
- Handling nextToken for list secrets
- Search for similar names
- Allow for different envs (look for env var?)
- Avoid string allocations
- Refresh method
- Lazy static option?
- Parameter store as an alternative for loading? As an additional macro?
