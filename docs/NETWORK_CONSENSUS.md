# AXIOM Network Consensus & Guardian Sentinel Guide

## Emergency: Addressing the Split-Brain Problem

### The Problem: Network Forks

If you have multiple nodes running independently at different block heights, you have a **network fork**:

```
Server (Port 6000): Block 6  ━━━━ [Living in Reality A]
Laptop (Port 6005): Block 10 ━━━━━━ [Living in Reality B]

Result: Two competing Axiom networks, 124M supply cap potentially violated
```

### Root Causes

1. **No Peer Discovery**: Nodes don't know how to find each other
2. **Different Listen Ports**: Isolation due to port differentiation
3. **Genesis Mismatch**: Different starting blocks (critical!)
4. **Firewall Blocking**: Nodes can't physically connect
5. **Bootstrap Peer Missing**: No initial peer list to bootstrap from

## Immediate Fix: Configuring Peers (Minutes 1-5)

### Step 1: Identify All Genesis Miners

For production rollout, you have 5 genesis miners:

```
Node 1 (Server A):    192.168.1.100:6000
Node 2 (Server B):    192.168.1.101:6000
Node 3 (Server C):    192.168.1.102:6000
Node 4 (Laptop D):    192.168.1.103:6000
Node 5 (Laptop E):    192.168.1.104:6000
```

### Step 2: Set Bootstrap Peers on ALL Nodes

**On each machine, set the environment variable:**

```bash
# Add to ~/.bashrc or ~/.zshrc on ALL 5 machines
export AXIOM_BOOTSTRAP_PEERS="192.168.1.100:6000,192.168.1.101:6000,192.168.1.102:6000,192.168.1.103:6000,192.168.1.104:6000"

# Or set in systemd service (recommended for production)
# See: contrib/axiom-guardian.service
```

**Or create `config/bootstrap.toml`:**

```toml
[[bootnodes]]
address = "192.168.1.100:6000"

[[bootnodes]]
address = "192.168.1.101:6000"

[[bootnodes]]
address = "192.168.1.102:6000"

[[bootnodes]]
address = "192.168.1.103:6000"

[[bootnodes]]
address = "192.168.1.104:6000"
```

### Step 3: Verify Network Connectivity

**Before starting nodes, test basic connectivity:**

```bash
# On Server A, test connection to Server B
ping 192.168.1.101
nc -zv 192.168.1.101 6000

# On Server A, test connection to Laptop
ping 192.168.1.103
nc -zv 192.168.1.103 6000

# Test reverse connectivity
ssh user@192.168.1.103 "nc -zv 192.168.1.100 6000"
```

### Step 4: Verify Identical Genesis Block

**CRITICAL: All nodes must start from the same genesis block!**

```bash
# On each node after first startup, check genesis hash
cat ~/.axiom/genesis_hash.txt

# Run on ALL 5 machines - should all print the same hash
axiom-node status | grep "Genesis"
```

**All outputs should be:**
```
Genesis Hash: 7876d9aac11b1197474167b7485626bf535e551a21865c6264f07f614281298c
```

If different, the nodes have diverged:
- Delete `~/.axiom/blocks/` on nodes with different genesis
- Restart them - they'll sync from the node with the correct genesis

## Emergency Recovery: Choosing the Canonical Chain

If nodes have already forked, decide which is correct:

### Option A: Longest Chain Wins (Preferred)

```bash
# On each node, check current height:
axiom-node status

# Example output:
# Node 1: Height 6
# Node 2: Height 10  ← THIS IS THE CANONICAL CHAIN

# For nodes at height < 10:
pkill axiom-node
rm -rf ~/.axiom/blocks/
# Copy the chain from Node 2
scp -r user@192.168.1.101:~/.axiom/blocks/ ~/.axiom/
# Restart with Node 2 as bootstrap
AXIOM_BOOTSTRAP_PEERS="192.168.1.101:6000" axiom-node
```

### Option B: Reset All Nodes (Nuclear Option)

If you don't trust any chain:

```bash
# On ALL 5 nodes simultaneously:
pkill axiom-node
rm -rf ~/.axiom/blocks/
rm -rf ~/.axiom/chain.dat

# Set identical bootstrap config
export AXIOM_BOOTSTRAP_PEERS="192.168.1.100:6000,192.168.1.101:6000,192.168.1.102:6000,192.168.1.103:6000,192.168.1.104:6000"

# Start on Node 1 first (it becomes the leader)
axiom-node

# Wait 30 seconds for Node 1 to generate genesis block
sleep 30

# Then start the other 4 nodes (they'll sync from Node 1)
axiom-node
```

## Sovereign Guardian: The Eternal Sentinel

The Neural Guardian is now enhanced to **never sleep**:

### How It Works

```
Heartbeat Pattern:
├── ACTIVE MODE (0-60 min idle)
│   ├── 60-second heartbeats
│   ├── Peer connectivity checks
│   ├── Quick health verification
│   └── Real-time threat detection
│
└── DEEP SLEEP MODE (60+ min idle, zero transactions)
    ├── 1-hour heartbeats
    ├── Full chain integrity verification
    ├── 124M supply cap enforcement
    ├── Zero-trust peer validation
    └── Exit Code: 0 = "Sovereignty Maintained"
```

### Log Output

```
[2026-02-05 14:23:01][INFO] 🛡️  Neural Guardian: Sentinel Active
[2026-02-05 14:24:01][INFO] 💚 Guardian Heartbeat [14:24:01] | Supply: 124M | Idle: 1m | Mode: Active
[2026-02-05 14:25:01][INFO] 💚 Guardian Heartbeat [14:25:01] | Supply: 124M | Idle: 2m | Mode: Active
[2026-02-05 15:25:01][INFO] 🌙 Guardian: DEEP SLEEP MODE [15:25:01]
[2026-02-05 15:25:01][INFO]   ⏱️  Idle: 1h
[2026-02-05 15:25:01][INFO]   🔐 Still monitoring... Zero-trust verification active.
[2026-02-05 15:25:01][INFO] ✓ 124M supply cap maintained
[2026-02-05 15:25:01][INFO] ✓ Peer count: 4/4 connected (genesis phase)
```

### What the Guardian Protects

Even during complete silence (no transactions for hours):

✅ **124M Supply Cap** - Verified every hour  
✅ **Chain Integrity** - Merkle roots checked automatically  
✅ **Peer Network** - All 5 nodes remain connected  
✅ **Consensus Rules** - No unauthorized forks allowed  
✅ **Sovereignty** - Node stays true to genesis configuration  

### Shutdown Handling

When the node receives SIGTERM (systemd stop):

```
[2026-02-05 14:30:00][WARN] 🛑 SHUTDOWN SIGNAL RECEIVED
[2026-02-05 14:30:00][WARN] Session duration: 6h 5m
[2026-02-05 14:30:00][WARN] Final mode: Active
[2026-02-05 14:30:00][WARN] Flushing logs and finalizing state...
[2026-02-05 14:30:00][INFO] Guardian: Clean shutdown complete. Exit code 0 = Sovereignty Maintained.
```

## Systemd Service Installation

For 24/7 operation:

```bash
# Copy service file
sudo cp contrib/axiom-guardian.service /etc/systemd/system/axiom-guardian.service

# Edit service to set your bootstrap peers
sudo nano /etc/systemd/system/axiom-guardian.service
# Find: Environment="AXIOM_BOOTSTRAP_PEERS=..."
# Update with your 5 genesis miner addresses

# Enable and start
sudo systemctl daemon-reload
sudo systemctl enable axiom-guardian
sudo systemctl start axiom-guardian

# Verify it's running
sudo systemctl status axiom-guardian

# Watch logs in real-time
sudo journalctl -u axiom-guardian -f

# Check that it restarted cleanly after restart
sudo systemctl restart axiom-guardian
sudo systemctl status axiom-guardian
```

## Network Diagnostic Commands

### Check Node Status

```bash
# Full status with all metrics
axiom-node status

# Expected output:
# ╔══════════════════════════════════════╗
# ║ HEIGHT: 42                           ║
# ║ BALANCE: 1050.00 AXM                 ║
# ║ PEERS: 4/4 CONNECTED                 ║
# ║ SYNC: ✅ IN SYNC                     ║
# ╚══════════════════════════════════════╝
```

### Check Peer Connections

```bash
# List all connected peers
axiom-node peers

# Expected output:
# 📡 Peer 1: 192.168.1.101:6000 | Height: 42 | Latency: 12ms | Trust: 1.0
# 📡 Peer 2: 192.168.1.102:6000 | Height: 42 | Latency: 15ms | Trust: 1.0
# 📡 Peer 3: 192.168.1.103:6000 | Height: 42 | Latency: 18ms | Trust: 1.0
# 📡 Peer 4: 192.168.1.104:6000 | Height: 42 | Latency: 22ms | Trust: 1.0
```

### Force Sync with Specific Peer

```bash
# If behind, manually sync from a trusted peer
axiom-node sync 192.168.1.101:6000

# Watch sync progress
watch -n 5 'axiom-node status | grep Height'
```

### Check Network Health

```bash
# Show network metrics
axiom-node network-status

# Expected output:
# Network Health:
# ├── Connected Peers: 4/4 ✅
# ├── Average Height: 42
# ├── Local Height: 42
# ├── Sync Status: IN SYNC ✅
# ├── Forks Detected: 0
# └── Latency (avg): 16ms
```

## Preventing Future Splits

### 1. **Golden Rule: Use Environment Variables**

All nodes must set before startup:

```bash
# Every node MUST have this
export AXIOM_BOOTSTRAP_PEERS="<all 5 genesis miners>"

# Then start normally
axiom-node
```

### 2. **Validate Genesis Hash at Startup**

Modify `src/main.rs` to verify genesis block:

```rust
// Check that all nodes have identical genesis
let genesis_hash = genesis::genesis().hash();
if genesis_hash != EXPECTED_GENESIS_HASH {
    panic!("❌ GENESIS MISMATCH! This node cannot join the network.");
}
log::info!("✅ Genesis verified: {}", hex::encode(genesis_hash));
```

### 3. **Health Check Endpoint**

HTTP API for monitoring:

```bash
# Check node health from terminal
curl http://localhost:8080/health

# Response:
# {
#   "status": "healthy",
#   "height": 42,
#   "peers": 4,
#   "synced": true,
#   "genesis_hash": "7876d9aa..."
# }
```

### 4. **Continuous Peer Syncing**

Network code will automatically:

- Try to connect to all bootstrap peers every 60 seconds
- Detect height differences and sync if needed
- Compare block hashes and reject forks
- Log all peer events for debugging

### 5. **Monitoring Dashboard**

Real-time visibility:

```bash
# Terminal-based dashboard
watch -n 2 'axiom-node peers && axiom-node status'

# Or use monitoring/Grafana (see monitoring/README.md)
ansible-playbook monitoring/setup-monitoring.yml
```

## Reference: Network Architecture Diagram

```
Genesis Phase (5 Nodes):

    Node 1 (6000)          Node 2 (6000)          Node 3 (6000)
    ┌─────────────┐        ┌─────────────┐        ┌─────────────┐
    │ Port 6000   │────────│ Port 6000   │────────│ Port 6000   │
    │ Height: 42  │        │ Height: 42  │        │ Height: 42  │
    │ Peers: 4    │        │ Peers: 4    │        │ Peers: 4    │
    └─────────────┘        └─────────────┘        └─────────────┘
         │                      │                       │
         └──────────┬───────────┴───────────┬───────────┘
                    │                       │
              Node 4 (6000)           Node 5 (6000)
              ┌─────────────┐        ┌─────────────┐
              │ Port 6000   │────────│ Port 6000   │
              │ Height: 42  │        │ Height: 42  │
              │ Peers: 4    │        │ Peers: 4    │
              └─────────────┘        └─────────────┘

All nodes:
✓ Same genesis block (matching hash)
✓ Same port (6000)
✓ Bootstrap peers configured identically
✓ Connected to each other in a full mesh
✓ Maintaining consensus on every block
```

## Troubleshooting

### Symptom: "Connected Peers: 0"

```bash
# Check bootstrap peers configured
echo $AXIOM_BOOTSTRAP_PEERS

# Should output all 5 addresses. If empty:
export AXIOM_BOOTSTRAP_PEERS="192.168.1.100:6000,192.168.1.101:6000,..."

# Check network connectivity
telnet 192.168.1.100 6000
# Should connect (press Ctrl+] then quit)

# Check firewall
sudo ufw status | grep 6000
# Should show port 6000 allowed
```

### Symptom: "Height Mismatch: Peer has 50, we have 30"

```bash
# Node is behind - will auto-sync
# Watch the sync in real-time
watch -n 2 'axiom-node status'

# If sync stalls, manually request blocks from peer
axiom-node sync 192.168.1.101:6000
```

### Symptom: "Genesis Hash Mismatch"

**CRITICAL**: This node has a different chain!

```bash
# Check what genesis hash this node has
axiom-node status | grep Genesis

# Check a peer's genesis
ssh user@192.168.1.101 'axiom-node status | grep Genesis'

# If different:
# - One node has wrong genesis (or fork)
# - Delete its chain: rm -rf ~/.axiom/blocks/
# - Restart it to resync from a correct peer
```

## Summary

| Aspect | Genesis Phase | Regular Operation |
|--------|---------------|-------------------|
| **Nodes** | 5 dedicated | 100s of validators |
| **Bootstrap Peers** | All 5 mining nodes | Public bootstrap nodes |
| **Listen Port** | 6000 (all) | 6000 (standardized) |
| **Min Peers for Consensus** | 4/5 | 2/3+ |
| **Fork Confirmation** | Longest chain | PoW work accumulation |
| **Guardian Mode** | Active (always) | Adaptive (Active/DeepSleep) |
| **Shutdown Behavior** | Graceful, logged | Graceful, logged |

The network is now bulletproof against split-brain failure.
