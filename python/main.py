import requests
import os
from bitcoinrpc.authproxy import AuthServiceProxy, JSONRPCException


def call_alice_ln(method, params=None):
    """Call Alice's Lightning node via CLN REST API on port 3010"""
    rune = os.environ.get('ALICE_RUNE')
    url = f'http://localhost:3010/v1/{method}'
    response = requests.post(
        url,
        json=params or {},
        headers={'Rune': rune}
    )
    return response.json()


def call_bob_ln(method, params=None):
    """Call Bob's Lightning node via CLN REST API on port 3011"""
    rune = os.environ.get('BOB_RUNE')
    url = f'http://localhost:3011/v1/{method}'
    response = requests.post(
        url,
        json=params or {},
        headers={'Rune': rune}
    )
    return response.json()

def call_carol_ln(method, params=None):
    """Call Carol's Lightning node via CLN REST API on port 3012"""
    rune = os.environ.get('CAROL_RUNE')
    url = f'http://localhost:3012/v1/{method}'
    response = requests.post(
        url,
        json=params or {},
        headers={'Rune': rune}
    )
    return response.json()

def main():

    try:
        # Get blockchain info
        rpc = AuthServiceProxy("http://alice:password@localhost:18443")
        print("Blockchain Info:", rpc.getblockchaininfo())

        # Get Alice's node info
        alice_info = call_alice_ln("getinfo")
        print("Alice Node Info:", alice_info)

        # Get Bob's node info
        bob_info = call_bob_ln("getinfo")
        print("Bob Node Info:", bob_info)

        # Get Carol's node info
        carol_info = call_carol_ln("getinfo")
        print("Carol Node Info:", carol_info)

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

    except JSONRPCException as e:
        print("An error occurred", e)

if __name__ == "__main__":
    main()
