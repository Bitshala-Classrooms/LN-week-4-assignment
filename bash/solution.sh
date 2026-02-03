#!/bin/bash
source bash/helper.sh

# Get blockchain info using bitcoin-cli
blockchain_info=$(bitcoin_cli getblockchaininfo)

# Print the blockchain info
echo "Blockchain Info: $blockchain_info"

alice_info=$(alice_ln_cli getinfo)
echo "Alice Node Info: $alice_info"

bob_info=$(bob_ln_cli getinfo)
echo "Bob Node Info: $bob_info"

carol_info=$(carol_ln_cli getinfo)
echo "Carol Node Info: $carol_info"

# Create a bitcoin wallet named 'mining_wallet' if it doesn't exist

# Generate a mining address and mine initial blocks

# Create and fund an on-chain address for Alice

# Create and fund an on-chain address for Bob

# Create and fund an on-chain address for Carol

# Mine blocks to confirm funding transactions

# Verify on-chain balance for Alice, Bob, and Carol

# Get node IDs for Alice, Bob, and Carol

# Connect them as peers

# Alice opens a 500,000 sat channel with Bob

# Bob opens a 300,000 sat channel with Carol

# Carol opens a 400,000 sat channel with Alice

# Mine at least 6 blocks to confirm channels

# Wait for channels to reach CHANNELD_NORMAL state

# Check opener's local balance for all channels

# Alice creates a BOLT11 invoice for 150,000 sats and description "Circular Rebalance"

# Extract the BOLT11 string and payment hash from the invoice

# Alice pays her own invoice

# Extract payment preimage and status

# Check local balances for all channels

# Write to out.txt:
# - Payment Hash
# - Payment Preimage
# - BOLT11 Invoice
# - local balance between Alice and Bob before CR in msat
# - local balance between Alice and Bob after CR in msat
# - local balance between Bob and Carol before CR in msat
# - local balance between Bob and Carol after CR in msat
# - local balance between Carol and Alice before CR in msat
# - local balance between Carol and Alice after CR in msat
