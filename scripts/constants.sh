#!/bin/bash

SECRET_NAME_WITH_PREFIX_DEV="/dev/secrets-manager-test-secret"
SECRET_VALUE_FOR_PREFIX_DEV='{ "firstKey": "firstValue", "secondKey": "secondValue" }'

SECRET_NAME_WITH_PREFIX_PROD="/prod/secrets-manager-test-secret"
SECRET_VALUE_FOR_PREFIX_PROD='{ "firstKey": "prodValue", "secondKey": "secondProdValue" }'

SECRET_NAME_WITHOUT_PREFIX="NoPrefixSecret"
SECRET_VALUE_FOR_NO_PREFIX='{ "thirdKey": "thirdValue" }'

INVALID_JSON_SECRET="InvalidSecret"
INVALID_JSON_VALUE='"firstKey": "prodValue"'
