#!/bin/bash

# Load variables from .env file
set -o allexport
source scripts/.env
set +o allexport

# Check for variables
if [ -z "$CONTRACT_ADDRESS" ] || [ -z "$NEW_COUNT" ]
then
    echo "CONTRACT_ADDRESS or NEW_COUNT are not set"
    echo "You can run the script by setting the variables at the beginning: CONTRACT_ADDRESS=0x NEW_COUNT=10 setCount.sh"
    exit 0
fi

# Call
echo "Setting the new counter..."
cast send --rpc-url $RPC_URL --private-key $PRIVATE_KEY $CONTRACT_ADDRESS "setCount(uint256)" $NEW_COUNT

new_counter=$(cast call --rpc-url $RPC_URL $CONTRACT_ADDRESS "get() (uint256)")
echo "New counter: $new_counter"

# CONTRACT_ADDRESS= NEW_COUNT=20 ./scripts/setCount.sh