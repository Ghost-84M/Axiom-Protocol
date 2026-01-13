#!/bin/bash

# Qubit Core - Decentralized 84M Launch Script
echo "--------------------------------------------------"
echo "🚀 INITIALIZING QUBIT CORE..."
echo "--------------------------------------------------"

# 1. Clean previous build artifacts
cargo clean

# 2. Build the optimized binary
echo "🛠️  Compiling release binary..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Build Successful."
    echo "--------------------------------------------------"
    echo "🏛️  STARTING DECENTRALIZED NODE..."
    echo "--------------------------------------------------"
    
    # 3. Execute the binary
    # Prefer `qubit-core` binary but fall back to `qubit` if present
    if [ -x ./target/release/qubit-core ]; then
        exec ./target/release/qubit-core
    elif [ -x ./target/release/qubit ]; then
        exec ./target/release/qubit
    else
        echo "❌ Built binary not found at ./target/release/qubit-core or ./target/release/qubit"
        exit 1
    fi
else
    echo "❌ Build Failed. Check the errors above."
    exit 1
fi
