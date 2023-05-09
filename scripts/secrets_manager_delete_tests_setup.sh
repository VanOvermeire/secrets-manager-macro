#!/bin/bash

# WARNING: This will *force-delete* the given secret

if ! source constants.sh; then
  exit 1
fi

function force_delete_secret() {
  echo "Force-deleting secret with name $1"
  aws secretsmanager delete-secret --force-delete-without-recovery --secret-id "$1" 2>&1 > /dev/null
}

force_delete_secret "$SECRET_NAME_WITH_PREFIX_DEV"
force_delete_secret "$SECRET_NAME_WITH_PREFIX_PROD"
force_delete_secret "$SECRET_NAME_WITHOUT_PREFIX"
force_delete_secret "$INVALID_JSON_SECRET"
