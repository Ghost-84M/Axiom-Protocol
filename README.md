# Qubit Protocol (84M) - Production-Ready Blockchain 🏛️

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-7%20passing-green.svg)](tests/)
[![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

**The 84,000,000 Sovereign Scarcity Engine** - A fully decentralized, production-ready blockchain with ZK-SNARK privacy, VDF consensus, and AI-powered network protection.

## 📋 Table of Contents

- [Overview](#-overview)
- [Key Features](#-key-features)
- [Architecture](#-architecture)
- [Installation](#-installation)
- [Quick Start](#-quick-start)
- [Usage](#-usage)
- [API Reference](#-api-reference)
- [Testing](#-testing)
- [Configuration](#-configuration)
- [Contributing](#-contributing)
- [License](#-license)

## 🌟 Overview

Qubit Protocol is a next-generation blockchain that combines:
- **Zero-Knowledge Privacy**: ZK-SNARK proofs for transaction anonymity
- **Time-Based Consensus**: VDF (Verifiable Delay Function) + PoW hybrid
- **AI Network Protection**: Neural Guardian for attack detection
- **Fixed Supply Economics**: 84M QBT with predictable halving schedule
- **Production-Ready**: Comprehensive testing, clean code, and enterprise features

### Core Philosophy
- **Absolute Scarcity**: Fixed 84M supply, no inflation, no governance
- **Mathematical Sovereignty**: Only provable math governs the network
- **Privacy by Default**: ZK-SNARKs mandatory for all transactions
- **Time as Consensus**: VDF ensures fair block production

## ✨ Key Features

### 🔐 Cryptographic Security
- **ZK-SNARK Proofs**: Zero-knowledge transaction validation
- **Ed25519 Signatures**: Industry-standard digital signatures
- **SHA-256 Hashing**: Secure block and transaction integrity
- **Nonce-based Replay Protection**: Account-based state with nonce tracking

### ⛏️ Consensus & Mining
- **Hybrid Consensus**: VDF (1-hour blocks) + PoW difficulty adjustment
- **Automatic Halving**: Mining rewards halve every 2.1 million blocks
- **Fair Distribution**: Genesis block with initial supply allocation
- **Network Synchronization**: P2P block and transaction propagation

### 🤖 AI Network Protection
- **Neural Guardian**: Local AI agent on every node
- **Attack Detection**: Identifies selfish mining, Sybil attacks, eclipse attacks
- **Peer Trust Scoring**: Dynamic reputation system for network peers
- **Anomaly Isolation**: Automatic quarantine of suspicious nodes

### 💰 Economics
- **Fixed Supply**: 84,000,000 QBT (84,000,000,000,000,000 smallest units)
- **Initial Reward**: 50 QBT per block
- **Halving Schedule**: Every 2,100,000 blocks (~4 years)
- **Deflationary Design**: Supply decreases over time

### 🔗 Network & Storage
- **P2P Networking**: libp2p with gossipsub protocol
- **Persistent Storage**: sled embedded database
- **Transaction Mempool**: Efficient transaction broadcasting
- **Bootstrap Discovery**: Automatic peer discovery and connection

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   CLI Wallet    │    │   Full Node     │    │   AI Engine     │
│                 │    │                 │    │                 │
│ • Key Generation│    │ • Block Mining  │    │ • Attack Detect │
│ • TX Creation   │    │ • TX Validation │    │ • Peer Scoring  │
│ • Balance Check │    │ • P2P Network   │    │ • Trust Analysis│
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
                    ┌─────────────────────┐
                    │   Core Protocol     │
                    │                     │
                    │ • ZK-SNARK Proofs   │
                    │ • VDF Consensus     │
                    │ • State Management  │
                    │ • Transaction Logic │
                    └─────────────────────┘
```

### Core Components

| Component | Technology | Purpose |
|-----------|------------|---------|
| **Consensus** | VDF + PoW | Fair block production |
| **Privacy** | ZK-SNARK | Transaction anonymity |
| **Networking** | libp2p | P2P communication |
| **Storage** | sled | Persistent data |
| **AI Security** | Neural Networks | Attack prevention |
| **Wallet** | Ed25519 | Key management |

## 🚀 Installation

### Prerequisites
- **Rust**: 1.70+ ([Install Rust](https://rustup.rs/))
- **System Dependencies**: Standard build tools

### Build from Source
```bash
# Clone the repository
git clone https://github.com/Ghost-84M/Qubit-Protocol-84m.git
cd Qubit-Protocol-84m

# Build in release mode for production
cargo build --release

# Optional: Run tests to verify installation
cargo test
```

### Binary Installation
```bash
# Download pre-built binaries (when available)
# TODO: Add binary releases
```

## 🏃 Quick Start

### 1. Start a Full Node
```bash
# Run the main node
cargo run --release --bin qubit

# The node will:
# - Connect to the P2P network
# - Start mining blocks
# - Begin transaction processing
# - Activate AI network protection
```

### 2. Create a Wallet
```bash
# Generate a new wallet
cargo run --release --bin qubit-wallet -- export

# Check your balance
cargo run --release --bin qubit-wallet -- balance <your-address>

# Create a transaction
cargo run --release --bin qubit-wallet -- send <to-address> <amount> <fee>
```

### 3. Check Network Status
```bash
# View supply information
cargo run --release --bin qubit-supply

# Monitor mining progress
# Check node logs for real-time updates
```

## 📖 Usage

### Node Operation
```bash
# Start mining node
cargo run --release

# Start non-mining node (relay only)
cargo run --release -- --relay-only

# Custom configuration
cargo run --release -- --config custom.toml
```

### Network Monitoring
The node displays real-time network status every 10 seconds:

```bash
--- 🏛️  QUBIT STATUS ---
⛓️  Height: 42 | Diff: 1000 | Trend: UP ⬆️
⏳ Time-Lock: 45m remaining | 🤖 AI Shield: ACTIVE
💰 Mined: 1,050.00 QBT | Remaining: 83,949,950.00 QBT | 1.25% of max supply
🌐 Connected Peers: 3 | Network: ACTIVE
------------------------
```

**Network Indicators:**
- **Connected Peers**: Number of active P2P connections
- **Peer Discovery**: Logs show when peers connect/disconnect
- **Block Verification**: Each block is verified by connected peers
- **Network Health**: Minimum 3 peers recommended for robust consensus

**Peer Events:**
```
🔗 Peer connected: 12D3KooWAbc... | Total peers: 3
🔌 Peer disconnected: 12D3KooWAbc... | Total peers: 2
🔎 mDNS discovered peer: 12D3KooWXyz...
👋 Identified peer: 12D3KooWDef... (qubit/1.0.0)
```

### Wallet Operations
```bash
# Export wallet address
./target/release/qubit-wallet export

# Check balance
./target/release/qubit-wallet balance <address>

# Send transaction
./target/release/qubit-wallet send <recipient> <amount> <fee>

# Show transaction history
./target/release/qubit-wallet history
```

### Development
```bash
# Run tests
cargo test

# Run specific test
cargo test test_transaction_validation

# Check code quality
cargo clippy

# Format code
cargo fmt

# Generate documentation
cargo doc --open
```

## 📚 API Reference

### Core Types

#### Transaction
```rust
pub struct Transaction {
    pub from: Address,      // Sender address
    pub to: Address,        // Recipient address
    pub amount: u64,        // Amount in smallest units
    pub fee: u64,          // Transaction fee
    pub nonce: u64,        // Account nonce
    pub signature: Vec<u8>, // Ed25519 signature
    pub zk_proof: Vec<u8>,  // ZK-SNARK proof
}
```

#### Block
```rust
pub struct Block {
    pub parent: [u8; 32],     // Parent block hash
    pub slot: u64,            // Time slot
    pub miner: Address,       // Miner address
    pub transactions: Vec<Transaction>,
    pub vdf_proof: Vec<u8>,   // VDF proof
    pub zk_proof: Vec<u8>,    // Miner ZK proof
    pub nonce: u64,           // PoW nonce
}
```

### Key Functions

#### Wallet Operations
```rust
// Create new wallet
let wallet = Wallet::new();

// Sign transaction
let signature = wallet.sign_transaction(&tx)?;

// Verify transaction
wallet.verify_transaction(&tx)?;
```

#### Node Operations
```rust
// Initialize node
let mut node = Timechain::new()?;

// Add transaction to mempool
node.add_transaction(tx)?;

// Mine new block
let block = node.mine_block()?;

// Validate block
node.validate_block(&block)?;
```

## 🧪 Testing

The project includes comprehensive integration tests covering all core functionality:

```bash
# Run all tests
cargo test

# Run with detailed output
cargo test -- --nocapture

# Run specific test categories
cargo test transaction  # Transaction tests
cargo test block        # Block creation tests
cargo test economics    # Economic model tests
cargo test wallet       # Wallet functionality tests
```

### Test Coverage

| Test Suite | Tests | Status |
|------------|-------|--------|
| **Transaction Validation** | 1 | ✅ Passing |
| **Block Creation** | 1 | ✅ Passing |
| **Chain Initialization** | 1 | ✅ Passing |
| **Economics** | 1 | ✅ Passing |
| **Wallet Balance** | 1 | ✅ Passing |
| **Transaction Creation** | 1 | ✅ Passing |
| **Block Hash** | 1 | ✅ Passing |
| **Total** | **7** | **✅ All Passing** |

## ⚙️ Configuration

### Default Configuration
```toml
# config.toml
[network]
listen_address = "/ip4/0.0.0.0/tcp/0"
bootstrap_peers = []

[mining]
enabled = true
threads = 4

[storage]
path = "qubit_chain.dat"

[ai]
neural_guardian_enabled = true
trust_threshold = 0.4
```

### Environment Variables
```bash
# Network configuration
export QUBIT_LISTEN_ADDR="/ip4/0.0.0.0/tcp/0"
export QUBIT_BOOTSTRAP_PEERS="peer1,peer2,peer3"

# Mining configuration
export QUBIT_MINING_ENABLED=true
export QUBIT_MINING_THREADS=4

# Storage configuration
export QUBIT_STORAGE_PATH="./data/qubit_chain.dat"
```

## 🤝 Contributing

We welcome contributions to the Qubit Protocol! Please follow these guidelines:

### Development Setup
```bash
# Fork and clone
git clone https://github.com/your-username/Qubit-Protocol-84m.git
cd Qubit-Protocol-84m

# Create feature branch
git checkout -b feature/your-feature

# Make changes, ensure tests pass
cargo test
cargo clippy

# Submit pull request
```

### Code Standards
- **Rust Edition**: 2021
- **Formatting**: `cargo fmt`
- **Linting**: `cargo clippy` (must pass)
- **Testing**: All new features require tests
- **Documentation**: Public APIs must be documented

### Areas for Contribution
- **ZK-SNARK Optimization**: Upgrade to full arkworks circuits
- **Network Enhancements**: Bootstrap nodes, peer discovery
- **AI Improvements**: Advanced attack detection algorithms
- **Performance**: Parallel processing, caching optimizations
- **Tools**: Block explorers, monitoring dashboards

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

```
MIT License

Copyright (c) 2026 Qubit Protocol Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.
```

## 🌐 Links & Resources

- **GitHub Repository**: [https://github.com/Ghost-84M/Qubit-Protocol-84m](https://github.com/Ghost-84M/Qubit-Protocol-84m)
- **Documentation**: [docs/](docs/) (generated with `cargo doc`)
- **Issues**: [GitHub Issues](https://github.com/Ghost-84M/Qubit-Protocol-84m/issues)
- **Discussions**: [GitHub Discussions](https://github.com/Ghost-84M/Qubit-Protocol-84m/discussions)

## ⚠️ Disclaimer

This software is experimental and provided "as is" without warranty of any kind. Use at your own risk. The Qubit Protocol is designed for educational and research purposes, though it implements production-grade security features.

---

**"The timeline is decentralized. No owners. No governance. Only math."**

*Built with ❤️ in Rust for the decentralized future.*

