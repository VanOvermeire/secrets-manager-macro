#!/bin/bash

# WARNING: will force-delete the given secret #

source constants.sh

echo "Force-deleting secret with name $SECRET_NAME"
aws secretsmanager delete-secret --secret-id --force-delete-without-recovery "$SECRET_NAME"
