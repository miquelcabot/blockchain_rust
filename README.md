# blockchain_rust

> A Rust implementation of a blockchain. This project aims to explore the fundamentals of blockchain technology using Rust's powerful system-level capabilities.

## Installation

To get started with `blockchain_rust`, ensure you have Rust and Cargo installed on your system. Then, clone this repository and build the project:

```bash
git clone https://github.com/miquelcabot/blockchain_rust.git
cd blockchain_rust
cargo build --release
```

## Usage
## Usage

After building the project, you can interact with the blockchain node using the following commands:

```bash
# Create a new blockchain
blockchain_rust createblockchain <ADDRESS>

# Create a new wallet
blockchain_rust createwallet

# Get the wallet balance of the target address
blockchain_rust getbalance <ADDRESS>

# Print local wallet addresses
blockchain_rust listaddresses

# Add a new block to the chain
blockchain_rust send <FROM> <TO> <AMOUNT> <MINE>

# Print all blocks in the blockchain
blockchain_rust printchain

# Rebuild UTXO index set
blockchain_rust reindexutxo

# Start a node
blockchain_rust startnode <ADDRESS>

# Print help for a command
blockchain_rust help <COMMAND>
```

For more detailed information on each command, you can use the `help` command followed by the name of the command you need more information about.

```bash
blockchain_rust help createblockchain
```

This will display detailed usage information for the `createblockchain` command.