# Secrets Manager Macro

A macro for using AWS Secrets Manager secrets in your application.

## Status

![Github Actions Status](https://github.com/VanOvermeire/secrets-manager-macro/actions/workflows/github-deploy.yml/badge.svg)

## Features

- Compile time checks
- Automatic loading of secrets into the chosen struct
- Sensible defaults where possible

## Setup and Usage

See [this readme](./secrets-manager-macro/README.md).

## Running the tests

The integration tests require that you have valid AWS credentials and an AWS Secrets Manager secret called 'GithubActionsTestSecret' in your AWS account.
You can use `secrets_manager_tests_setup.sh` in `scripts` to create this secret with the correct secret value. 
`secrets_manager_delete_tests_setup.sh` will clean up these test secrets, if you want to save costs.

At the time of writing, a secret will cost you 40 dollar cents per month, plus 5 cents for 10 000 API calls (which will probably round down to 0 cents).

## TODOs

- GitHub actions publish

- Attribute for changing secret name
- Only allow the passed in `envs` when calling `new`?
- Check all env contents (currently assumes dev for validation)
- Attribute for checking a *selection* of envs + allow disabling of compile time checks (more useful once you can add fields to the struct)? Or 'saving' of a check?
- Refresh method (and keep a validity timestamp?)
- Allow selection of secrets by adding fields to the struct

## Improvements, extensions

- One or two clones to get rid of
- Lazy static as an option?
- Parameter store as an alternative for loading? Maybe as an additional macro
