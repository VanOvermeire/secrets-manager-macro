#!/bin/bash

set -eo pipefail

lambda=$(aws cloudformation describe-stack-resources --stack-name SecretsManagerE2ETest --query 'StackResources[?contains(ResourceType, `AWS::Lambda::Function`) == `true`].PhysicalResourceId' --output text)

echo "Invoking $lambda"
aws lambda invoke --function-name "$lambda" --payload "{}" outfile 2>&1 > /dev/null

if ! diff --strip-trailing-cr expected outfile ; then
  echo "Result does not match expected"
  exit 1
fi
