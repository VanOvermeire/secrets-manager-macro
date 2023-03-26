#!/bin/bash

source constants.sh

aws secretsmanager get-secret-value --secret-id "${SECRET_NAME}" 2>&1 > /dev/null

if [ $? -ne 0 ]; then
  echo "No secret with name ${SECRET_NAME} yet - creating it with expected values"
  aws secretsmanager create-secret --name "${SECRET_NAME}" --secret-string "${SECRET_VALUE}" 2>&1 > /dev/null
fi
