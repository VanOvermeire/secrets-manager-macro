# Secret Manager Macro

A macro for using AWS Secret Manager secrets in your application.

## Status

![Github Actions Status](https://github.com/VanOvermeire/secret-manager-macro/actions/workflows/github-deploy.yml/badge.svg)

## Features

- Compile time checks
- Automatic loading of secrets into the chosen struct
- Sensible defaults where possible

## Setup and Usage

See [this readme](./secret-manager-macro/README.md).

## Running the tests

The integration tests require that you have valid AWS credentials and an AWS Secret Manager secret called 'GithubActionsTestSecret' in your AWS account.
You can use `secret_manager_tests_setup.sh` in `scripts` to create this secret with the correct secret value. 
`secret_manager_delete_tests_setup.sh` will clean up these test secrets, if you want to save costs.

At the time of writing, a secret will cost you 40 dollar cents per month, plus 5 cents for 10 000 API calls (which will probably round down to 0 cents).

## TODOs

- GitHub actions publish
- GitHub end-to-end test with deploy lambda and check output

- Attribute for changing secret name
- Only allow the passed in `envs` when calling `new`?
- Check all env contents
- Attribute for checking a *selection* of envs + allow disabling of compile time checks (more useful once you can add fields to the struct)? Or 'saving' of a check?
- Refresh method (and keep a validity timestamp?)
- Allow selection of secrets by adding fields to the struct

## Improvements, extensions

- 'service error' as message when no aws credentials
- One or two clones to get rid of
- Lazy static as an option?
- Parameter store as an alternative for loading? Maybe as an additional macro
