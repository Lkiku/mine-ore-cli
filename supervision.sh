#!/bin/bash

# Set default values
# Multiple RPC supported
RPC_URLS=("https://cold-bitter-wildflower.solana-mainnet.quiknode.pro/c5f4ba498f7a5d3d2e78a94e3a3a423ff8e6c3f1/" "https://patient-responsive-shard.solana-mainnet.quiknode.pro/a41a6faa3017a567c148a796bf88044b6cf377e9/")
KEYS_FILE="keys.txt"
DEFAULT_FEE=100000
DEFAULT_THREADS=20

# Assign arguments with defaults
FEE=${2:-$DEFAULT_FEE}
THREADS=${3:-$DEFAULT_THREADS}

echo "Starting"

while IFS= read -r KEY; do
  # Loop indefinitely for each key
  (
    while true; do
      echo "Starting the process for key: ${KEY:0:8}..."
      RPC_URL=${RPC_URLS[$RANDOM % ${#RPC_URLS[@]}]}
      echo "Using RPC URL: ${RPC_URL}"
      
      # Execute the command in background
      "./target/release/ore" --rpc ${RPC_URL} --keypair "${KEY}" --priority-fee ${FEE} mine --threads ${THREADS} &
      
      PID=$!
      wait $PID
      [ $? -eq 0 ] && break
      
      echo "Process for key: ${KEY:0:8} exited with an error. Restarting in 5 seconds..."
      sleep 5
    done
  ) &
done < "$KEYS_FILE"

# Wait for all background processes to finish
wait
