#!/bin/bash

if ! source constants.sh; then
  exit 1
fi

function create_secret_if_not_exist() {
  if ! aws secretsmanager get-secret-value --secret-id "$1" 2>&1 > /dev/null; then
    echo "No secret with name ${SECRET_NAME} yet - creating it with expected values"
    aws secretsmanager create-secret --name "$1" --secret-string "$2" 2>&1 > /dev/null
  fi
}

create_secret_if_not_exist "${SECRET_NAME_WITH_PREFIX_DEV}" "${SECRET_VALUE_FOR_PREFIX_DEV}"
create_secret_if_not_exist "${SECRET_NAME_WITH_PREFIX_PROD}" "${SECRET_VALUE_FOR_PREFIX_PROD}"
create_secret_if_not_exist "${SECRET_NAME_WITHOUT_PREFIX}" "${SECRET_VALUE_FOR_NO_PREFIX}"
