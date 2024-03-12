#!/bin/bash

# Load variables from .env file
set -o allexport
source scripts/.env
set +o allexport

# Check for variables
if [ -z "$CONTRACT_ADDRESS" ] 
then
    echo "CONTRACT_ADDRESS is not set"
    echo "You can run the script by setting the variables at the beginning: CONTRACT_ADDRESS=0x increment.sh"
    exit 0
fi

# Call
echo "Incrementing the counter..."
cast send --rpc-url $RPC_URL --private-key $PRIVATE_KEY $CONTRACT_ADDRESS "increment()"

new_counter=$(cast call --rpc-url $RPC_URL $CONTRACT_ADDRESS "get() (uint256)")
echo "New counter: $new_counter"

# CONTRACT_ADDRESS= ./scripts/increment.sh