use bitcoincore_rpc::{Auth, Client as BitcoinClient, RpcApi};
use reqwest::blocking::Client;
use serde_json::Value;

/// Call Alice's Lightning node via CLN REST API on port 3010
fn call_alice_ln(method: &str, params: Value) -> Result<Value, Box<dyn std::error::Error>> {
    let rune = std::env::var("ALICE_RUNE")?;
    let url = format!("http://localhost:3010/v1/{}", method);

    let client = Client::new();
    let response = client
        .post(&url)
        .json(&params)
        .header("Rune", rune)
        .send()?
        .json::<Value>()?;

    Ok(response)
}

/// Call Bob's Lightning node via CLN REST API on port 3011
fn call_bob_ln(method: &str, params: Value) -> Result<Value, Box<dyn std::error::Error>> {
    let rune = std::env::var("BOB_RUNE")?;
    let url = format!("http://localhost:3011/v1/{}", method);

    let client = Client::new();
    let response = client
        .post(&url)
        .json(&params)
        .header("Rune", rune)
        .send()?
        .json::<Value>()?;

    Ok(response)
}

/// Call Carols's Lightning node via CLN REST API on port 3012
fn call_carol_ln(method: &str, params: Value) -> Result<Value, Box<dyn std::error::Error>> {
    let rune = std::env::var("CAROL_RUNE")?;
    let url = format!("http://localhost:3012/v1/{}", method);

    let client = Client::new();
    let response = client
        .post(&url)
        .json(&params)
        .header("Rune", rune)
        .send()?
        .json::<Value>()?;

    Ok(response)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get blockchain info
    let rpc = BitcoinClient::new(
        "http://localhost:18443",
        Auth::UserPass("alice".to_string(), "password".to_string()),
    )?;

    println!("Blockchain Info: {:?}", rpc.get_blockchain_info()?);
    // Get Alice's node info
    let alice_info = call_alice_ln("getinfo", Value::Null)?;
    println!("Alice Node Info: {:?}", alice_info);

    // Get Bob's node info
    let bob_info = call_bob_ln("getinfo", Value::Null)?;
    println!("Bob Node Info: {:?}", bob_info);


    // Get Carol's node info
    let carol_info = call_carol_ln("getinfo", Value::Null)?;
    println!("Carol Node Info: {:?}", carol_info);

    // Create a bitcoin wallet named 'mining_wallet' if it doesn't exist

    // Generate a mining address and mine initial blocks

    // Create and fund an on-chain address for Alice

    // Create and fund an on-chain address for Bob

    // Create and fund an on-chain address for Carol

    // Mine blocks to confirm funding transactions

    // Verify on-chain balance for Alice, Bob, and Carol

    // Get node IDs for Alice, Bob, and Carol

    // Connect them as peers

    // Alice opens a 500,000 sat channel with Bob

    // Bob opens a 300,000 sat channel with Carol

    // Carol opens a 400,000 sat channel with Alice

    // Mine at least 6 blocks to confirm channels

    // Wait for channels to reach CHANNELD_NORMAL state

    // Check opener's local balance for all channels

    // Alice creates a BOLT11 invoice for 150,000 sats and description "Circular Rebalance"

    // Extract the BOLT11 string and payment hash from the invoice

    // Alice pays her own invoice

    // Extract payment preimage and status

    // Check local balances for all channels

    // Write to out.txt:
    // - Payment Hash
    // - Payment Preimage
    // - BOLT11 Invoice
    // - local balance between Alice and Bob before CR in msat
    // - local balance between Alice and Bob after CR in msat
    // - local balance between Bob and Carol before CR in msat
    // - local balance between Bob and Carol after CR in msat
    // - local balance between Carol and Alice before CR in msat
    // - local balance between Carol and Alice after CR in msat

    Ok(())
}
