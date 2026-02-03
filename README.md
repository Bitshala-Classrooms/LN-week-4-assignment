# Mastering Lightning Network - Week 4: Circular Rebalancing & Channel Liquidity

## Overview

In this exercise you will:

1. **Set up** Bitcoin Core and Core Lightning (CLN) nodes using Docker.
2. **Interact** with both Bitcoin Core and Lightning nodes on `regtest`.
3. **Build** a circular channel topology across three Lightning nodes and **Execute** a circular rebalancing payment where Alice pays herself through the ring.
4. **Output** a report file `out.txt` in the **current directory**
5. **Target Locations** for the solution code for each language are given below:
   - Bash: [solution.sh](./bash/solution.sh)
   - JavaScript: [index.js](./javascript/index.js)
   - Python: [main.py](./python/main.py)
   - Rust: [main.rs](./rust/src/main.rs)

## Problem Statement

Using a 3-node network, create a circular topology by opening channels between **Alice and Bob** (500,000 sats) ; **Bob and Carol** (300,000 sats) ; **Carol and Alice** (400,000 sats)

Execute a circular rebalancing payment where Alice pays herself **150,000 satoshis** by routing the payment through Bob and Carol.

## Solution Requirements

You need to write code in any one of `bash`, `javascript`, `python` or `rust` that will do the following:

### Setup - Docker Compose

The assignment uses Docker Compose to run both Bitcoin Core and Core Lightning nodes. The configuration is provided in [docker-compose.yml](./docker-compose.yml).

Services:
- **bitcoind**: Bitcoin Core node running on regtest
  - RPC port: 18443
  - RPC credentials: alice/password
- **alice**: Core Lightning node
  - Lightning P2P port: 9735 (host: 9735)
  - CLN REST port: 3010
- **bob**: Core Lightning node
  - Lightning P2P port: 9735 (host: 9736)
  - CLN REST port: 3011
- **carol**: Core Lightning node
  - Lightning P2P port: 9735 (host: 9737)
  - CLN REST port: 3012

To start the services:
```bash
docker compose up -d
```
To stop the services:
```bash
docker compose down -v
```

### Node Interaction - Choose ONE Language

Implement the tasks in exactly one of the language-specific directories: `bash`, `javascript`, `python`, or `rust`.

Each implementation uses helper functions located in the directories.

Your program must:

- Create a bitcoin mining wallet and mine initial blocks
- Fund Alice's, Bob's and Carol's on-chain wallets from the mining wallet
- Mine blocks to confirm funding transactions and verify balances
- Get node IDs for Alice, Bob, and Carol
- Connect peers (Alice -> Bob, Bob -> Carol, Carol -> Alice)
- Open channels as specified in problem statement
- Mine blocks to confirm channels
- Wait for channels to reach `CHANNELD_NORMAL` state
- Record opener's local balance for each Channel
- Alice generates a 150,000 sat invoice with description "Circular Rebalance"
- Alice pays her own invoice
- Verify the payment status is `complete`
- Extract payment hash and preimage
- Record local balance for each Channel
- Write output to `out.txt` in the specified format

### Output

Output the following invoice details to `out.txt` in the root directory. Each attribute should be on a new line:
- Payment Hash (64 char hex)
- Payment Preimage (64 char hex)
- BOLT11 Invoice string
- local balance between **Alice and Bob** before circular rebalancing
- local balance between **Alice and Bob** after circular rebalancing
- local balance between **Bob and Carol** before circular rebalancing
- local balance between **Bob and Carol** after circular rebalancing
- local balance between **Carol and Alice** before circular rebalancing
- local balance between **Carol and Alice** after circular rebalancing

> **Note**: Exact msat values will vary due to channel reserves and routing fees. Key property to note is, each "after" value is less than its corresponding "before" value, i.e. liquidity moved along the payment path.

Sample output file:
```
a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2
f1e2d3c4b5a6f7e8d9c0b1a2f3e4d5c6b7a8f9e0d1c2b3a4f5e6d7c8b9a0f1e2
lnbcrt1500u1pn...
499019000
349019000
298019000
148019000
399019000
249019000
```

## Code Structure

Each language implementation follows a consistent pattern:

```
<language>/
  ├── helper.<ext>      # Helper functions for Bitcoin/Lightning CLI
  ├── main.<ext>        # Main implementation
  └── run-<language>.sh # Script to run the implementation
```

Helper functions abstract Docker CLI interactions:
- `bitcoin_cli(command)`: Execute bitcoin-cli commands via Docker
- `alice_ln_cli(command)`: Execute lightning-cli for Alice via Docker
- `bob_ln_cli(command)`: Execute lightning-cli for Bob via Docker
- `carol_ln_cli(command)`: Execute lightning-cli for Carol via Docker

## Local Testing

### Prerequisites

| Language       | Prerequisite packages                  |
| -------------- |----------------------------------------|
| **Bash**       | Docker, Docker Compose, `jq`           |
| **JavaScript** | Docker, Docker Compose, Node.js ≥ 20   |
| **Python**     | Docker, Docker Compose, Python ≥ 3.9   |
| **Rust**       | Docker, Docker Compose, Rust toolchain |

### Setup Steps

1. **Install Docker and Docker Compose**
   ```bash
   # Follow Docker installation guide for your OS
   # https://docs.docker.com/get-docker/
   ```

2. **Install Language Components**

   #### Bash
   - **Version** 4.0 or higher (usually pre-installed on Linux/macOS)
    ```bash
    # check version
    bash --version
    
    # to install jq [JSON processor to parse JSON responses]
    sudo apt-get update && sudo apt-get install -y jq       # Ubuntu/Debian
    brew install jq                                         # macOS
    ```

   #### JavaScript
   - **Node.js Version** 20.x or higher

   ```bash
    # check version
    node --version
    npm --version
    
    # install nvm
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
    nvm install 20
    nvm use 20
    
    # install project dependencies
    cd javascript
    npm install
    ```

   #### Python
   - **Version** 3.9 or higher
    ```bash
    # check version
    python3 --version
    pip3 --version

    # install python
    sudo apt-get update && sudo apt-get install -y python3 python3-pip python3-venv       # Ubuntu/Debian
    brew install python@3.9                                                               # macOS

    # install required dependencies
    pip3 install requests python-bitcoinrpc
    ```

   #### Rust
   - **Version** 1.70.0 or higher
    ```bash
    # check version
    rustc --version
    cargo --version

    # installation via rustup
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh                        # Linux/macOS
    # Follow the prompts, then reload your shell.
    source $HOME/.cargo/env

    # build essentials
    sudo apt-get install -y build-essential pkg-config libssl-dev                         # Ubuntu/Debian

    # Build the project
    cd rust
    cargo build
    ```

3. **Start the nodes**
   ```bash
   docker compose up -d
   ```

### Local Testing Steps
It's a good idea to run the whole test locally to ensure your code is working properly.

- Uncomment the specific line in [run.sh](./run.sh) corresponding to your language of choice.
- Grant execution permission to [test.sh](./test.sh), by running `chmod +x ./test.sh`.
- Execute `./test.sh`.
- The test script will run your script and verify the output. If the test script passes, you have successfully completed the challenge and are ready to submit your solution.

### Common Issues

- If docker containers not running ensure `docker-compose up -d` completed successfully
- Make sure Docker daemon is running and you have permissions using `docker ps`
- Ensure `out.txt` has exactly 9 lines in the correct order
- The autograder will run the test script on an Ubuntu 22.04 environment. Make sure your script is compatible with this environment.
- If you are unable to run the test script locally, you can submit your solution and check the results on the Github.

## Submission

- Commit all code inside the appropriate language directory and the modified `run.sh`.
  ```
  git add .
  git commit -m "Week 4 solution"
  ```
- Push to the main branch:
  ```
    git push origin main
  ```
- The autograder will run your script against a test script to verify the functionality.
- Check the status of the autograder on the Github Classroom portal to see if it passed successfully or failed. Once you pass the autograder with a score of 100, you have successfully completed the challenge.
- You can submit multiple times before the deadline. The latest submission before the deadline will be considered your final submission.
- You will lose access to the repository after the deadline.

## Evaluation Criteria

| Area                   | Weight      | Description                                                                                                                         |
| ---------------------- | ----------- |-------------------------------------------------------------------------------------------------------------------------------------|
| **Autograder**         | **Primary** | Your code must pass the autograder [test script](./test/test.spec.ts).                                                                              |
| **Explainer comments** | Required    | Include comments explaining each step of your code.                                                                                 |
| **Code quality**       | Required    | Your code should be well-organized, commented, and adhere to best practices like idiomatic style, meaningful names, error handling. |

### Plagiarism Policy
Our plagiarism detection checker thoroughly identifies any instances of copying or cheating. Participants are required to publish their solutions in the designated repository, which is private and accessible only to the individual and the administrator. Solutions should not be shared publicly or with peers. In case of plagiarism, both parties involved will be directly disqualified to maintain fairness and integrity.

### AI Usage Disclaimer
You may use AI tools like ChatGPT to gather information and explore alternative approaches, but avoid relying solely on AI for complete solutions. Verify and validate any insights obtained and maintain a balance between AI assistance and independent problem-solving.

## Why These Restrictions?
These rules are designed to enhance your understanding of the technical aspects of Bitcoin. By completing this assignment, you gain practical experience with the technology that secures and maintains the trustlessness of Bitcoin. This challenge not only tests your ability to develop functional Bitcoin applications but also encourages deep engagement with the core elements of Bitcoin technology.
