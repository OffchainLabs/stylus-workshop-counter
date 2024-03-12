#!/bin/bash

# Load variables from .env file
set -o allexport
source scripts/.env
set +o allexport

# Check for variables
if [ -z "$CONTRACT_ADDRESS" ] 
then
    echo "CONTRACT_ADDRESS is not set"
    echo "You can run the script by setting the variables at the beginning: CONTRACT_ADDRESS=0x getCount.sh"
    exit 0
fi

# Call
current_counter=$(cast call --rpc-url $RPC_URL $CONTRACT_ADDRESS "get() (uint256)")
echo "Counter: $current_counter"

# CONTRACT_ADDRESS= ./scripts/getCount.sh