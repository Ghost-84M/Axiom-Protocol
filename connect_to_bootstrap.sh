#!/bin/bash

# Axiom Protocol - Bootstrap Node Connection Script
# Usage: bash connect_to_bootstrap.sh
# This script automatically configures your node to connect to the mainnet bootstrap node

set -e

BOOTSTRAP_IP="34.10.172.207"
BOOTSTRAP_PORT="6000"
BOOTSTRAP_PEER="12D3KooWAzD3QjhHMamey1XuysPovzwXyAZy9VzpZmQN7GkrURWU"
BOOTSTRAP_MULTIADDR="/ip4/$BOOTSTRAP_IP/tcp/$BOOTSTRAP_PORT/p2p/$BOOTSTRAP_PEER"

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║     AXIOM PROTOCOL - BOOTSTRAP NODE CONNECTION SCRIPT         ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""
echo "🌐 Bootstrap Node Information:"
echo "   IP Address:    $BOOTSTRAP_IP"
echo "   Port:          $BOOTSTRAP_PORT"
echo "   PeerId:        $BOOTSTRAP_PEER"
echo ""

# Check if we're in the Axiom-Protocol directory
if [ ! -f "Cargo.toml" ] || [ ! -d "config" ]; then
    echo "❌ Error: Please run this script from the Axiom-Protocol directory"
    echo "   cd ~/Axiom-Protocol && bash connect_to_bootstrap.sh"
    exit 1
fi

echo "📋 Select connection method:"
echo ""
echo "  1) Edit config/bootstrap.toml (Recommended for production)"
echo "  2) Use environment variable (Quick testing)"
echo "  3) Show multiaddr only (Manual setup)"
echo ""
read -p "Enter choice (1-3): " choice

case $choice in
    1)
        echo ""
        echo "✏️  Updating config/bootstrap.toml..."
        
        # Backup existing config
        cp config/bootstrap.toml config/bootstrap.toml.bak
        echo "   ✅ Backup created: config/bootstrap.toml.bak"
        
        # Create new bootstrap config
        cat > config/bootstrap.toml << EOF
# Bootstrap nodes for Axiom mainnet
# These are initial peers to connect to for network discovery
# Format: /ip4/<ip>/tcp/<port>/p2p/<peer_id>

# DEPLOYMENT INSTRUCTIONS:
# For MAINNET: The bootstrap nodes below are pre-configured
# For TESTNET/LOCAL: Leave empty and use mDNS discovery
# For CUSTOM: Override via environment variable:
#    export AXIOM_BOOTSTRAP_PEERS="/ip4/x.x.x.x/tcp/6000/p2p/peer_id"
#    cargo run --release

# ========================================
# AXIOM BOOTSTRAP NODES (February 2026)
# ========================================

bootnodes = [
    "$BOOTSTRAP_MULTIADDR",  # Primary bootstrap node
]

# ========================================
# ADDITIONAL BOOTSTRAP NODES
# Add more bootstrap nodes here for redundancy
# ========================================
EOF
        
        echo "   ✅ config/bootstrap.toml updated"
        echo ""
        echo "🚀 Ready to run! Execute:"
        echo "   cargo build --release"
        echo "   ./target/release/axiom"
        echo ""
        ;;
        
    2)
        echo ""
        echo "📦 Setting environment variable and running..."
        echo ""
        export AXIOM_BOOTSTRAP_PEERS="$BOOTSTRAP_MULTIADDR"
        echo "✅ Environment variable set:"
        echo "   AXIOM_BOOTSTRAP_PEERS=$AXIOM_BOOTSTRAP_PEERS"
        echo ""
        
        read -p "Build and run now? (y/n): " run_now
        if [ "$run_now" = "y" ] || [ "$run_now" = "Y" ]; then
            cargo build --release
            cargo run --release
        else
            echo ""
            echo "To connect manually, run:"
            echo "   export AXIOM_BOOTSTRAP_PEERS=\"$BOOTSTRAP_MULTIADDR\""
            echo "   cargo build --release"
            echo "   cargo run --release"
        fi
        ;;
        
    3)
        echo ""
        echo "📋 Multiaddr for manual configuration:"
        echo ""
        echo "   $BOOTSTRAP_MULTIADDR"
        echo ""
        echo "Use this address in:"
        echo "   • config/bootstrap.toml (bootnodes array)"
        echo "   • AXIOM_BOOTSTRAP_PEERS environment variable"
        echo "   • Docker/K8s configs"
        echo ""
        ;;
        
    *)
        echo "❌ Invalid choice. Please enter 1, 2, or 3."
        exit 1
        ;;
esac

echo ""
echo "📚 For more information, see:"
echo "   • BOOTSTRAP_DEPLOYMENT.md - Comprehensive deployment guide"
echo "   • README.md - General setup instructions"
echo "   • docs/ - Additional documentation"
echo ""
echo "✅ Configuration complete!"
