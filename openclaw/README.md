# OpenClaw - Axiom Protocol Automation

Agent-based automation for Axiom Protocol operations.

## Overview

This module integrates three powerful capabilities using autonomous agents:

### 1. 🔬 Ceremony Master
**ZK-Ceremony Phase 2 Automation**

Automatically orchestrates multi-miner proof generation without human coordination:

```python
from openclaw import CeremonyMaster

# Initialize with miners
ceremony = CeremonyMaster([
    {"id": "miner_1", "name": "Miner Alpha", "contact": "miner1@axiom.local"},
    {"id": "miner_2", "name": "Miner Beta", "contact": "miner2@axiom.local"},
    {"id": "miner_3", "name": "Miner Gamma", "contact": "miner3@axiom.local"},
])

# Start fully automated coordination
await ceremony.start_ceremony()

# Receive and verify contributions
await ceremony.receive_zkey("path/to/zkey", "miner_1", zkey_hash)

# Get status anytime
status = await ceremony.get_status()
```

**Features:**
- Sequential miner coordination
- Automatic handoff notifications
- Hash verification of contributions
- Real-time progress tracking
- Complete failure resilience

---

### 2. 🏥 Node Health Monitor
**Network Surveillance & Alerting**

Continuously monitors peer connectivity, latency, and system health with intelligent alerting:

```python
from openclaw import NodeHealthMonitor

# Create monitor for your node
monitor = NodeHealthMonitor("axiom_node_main", check_interval_seconds=30)

# Start background monitoring
await monitor.start_monitoring()

# Get current health snapshot
health = await monitor.get_current_health()

# Query recent alerts
alerts = await monitor.get_recent_alerts(count=20)

# Export metrics for analysis
await monitor.export_metrics("metrics.json")
```

**Monitored Metrics:**
- Peer count and connectivity
- Latency per peer (milliseconds)
- Memory and disk usage
- Uptime and error rates
- Connection stability

**Alert Types:**
- 🚨 Low Peer Count (< 2 peers)
- ⚠️ High Latency (> 200ms)
- ⚠️ Memory Pressure (> 80%)
- 🚨 Low Disk Space (< 10GB)

---

### 3 🌐 Agent Internet Integration
**Multi-Platform Deployment**

Deploy Axiom skills to major agent platforms for decentralized automation:

```python
from openclaw import AgentInternetIntegration, create_axiom_agents

# Create agents for all platforms
agents = create_axiom_agents()

# Deploy to Moltbook
moltbook_config = agents['moltbook']

# Deploy to Eliza
eliza_config = agents['eliza']

# Deploy to Autonolas
autonolas_config = agents['autonolas']
```

**Supported Platforms:**
- **Moltbook** - Decentralized AI agent marketplace
- **Eliza** - Modular character-based agent framework
- **Autonolas** - Autonomous service network with token rewards

**Deployment Types:**
- Standalone agents (Moltbook, Eliza)
- Autonomous services (Autonolas)
- Composable skills for multi-platform coordination

---

## Quick Start

### 1. Initialize Manager
```python
from openclaw import AxiomOpenClawManager

manager = AxiomOpenClawManager()
manager.load_config("axiom_openclaw_config.json")

await manager.initialize_ceremony_master()
await manager.initialize_health_monitor()
await manager.initialize_agent_internet()
```

### 2. Run Interactive Console
```bash
python3 openclaw/manager.py
```

Commands:
- `start-ceremony` - Begin ZK ceremony automation
- `monitor-status` - Show system status
- `ceremony-history` - View handoff history
- `health-report` - Network health report
- `export-config` - Export deployment config

### 3. Configuration (axiom_openclaw_config.json)
```json
{
  "ceremony": {
    "enabled": true,
    "miners": [
      {
        "id": "miner_1",
        "name": "Miner Alpha",
        "contact": "miner1@axiom.local",
        "endpoint": "http://localhost:8001"
      }
    ]
  },
  "monitoring": {
    "enabled": true,
    "node_id": "axiom_node_main",
    "check_interval_seconds": 30
  },
  "agent_internet": {
    "enabled": true,
    "platforms": ["moltbook", "eliza", "autonolas"]
  }
}
```

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│          Axiom Protocol Network (libp2p)                    │
└─────────────────────────────────────────────────────────────┘
                              ▲
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
        ▼                     ▼                     ▼
┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐
│ Ceremony Master  │  │ Health Monitor   │  │ Agent Internet   │
│                  │  │                  │  │                  │
│ • Coordinates    │  │ • Tracks peers   │  │ • Moltbook       │
│ • Verifies       │  │ • Alerts issues  │  │ • Eliza          │
│ • Auto-handoff   │  │ • Exports data   │  │ • Autonolas      │
└──────────────────┘  └──────────────────┘  └──────────────────┘
        │                     │                     │
        └─────────────────────┼─────────────────────┘
                              │
                              ▼
                    ┌──────────────────┐
                    │ AxiomOpenClaw    │
                    │ Manager          │
                    │                  │
                    │ • Config mgmt    │
                    │ • Component init │
                    │ • Interactive UI │
                    └──────────────────┘
```

---

## Integration with Axiom Core

### libp2p Networking
All three components integrate with Axiom's libp2p network:

**Ceremony Master:**
- Uses RequestResponse protocol to communicate with miners
- Falls back to Gossipsub for broadcasts
- Registered with Kademlia DHT

**Health Monitor:**
- Listens to Identify protocol for peer info
- Tracks ConnectionHandler events
- Integrates with Kademlia peer discovery

**Agent Internet:**
- Exports network metrics to deployed agents
- Agents receive peer/node data via REST APIs
- Enables off-chain coordination

### Data Flow
```
Axiom Node (libp2p)
    ↓
    ├→ Event Stream
    │    └→ Health Monitor (collect metrics)
    │         └→ Alerts (if issues detected)
    │              └→ Agent Internet (notify platforms)
    │
    ├→ RequestResponse/Gossipsub
    │    └→ Ceremony Master (coordinate miners)
    │         └→ Mine proofs
    │              └→ Receive .zkey
    │
    └→ Peer Discovery
         └→ Kademlia DHT
              └→ Health Monitor (update peer list)
```

---

## Real-World Example: Full Ceremony + Monitoring

```python
import asyncio
from openclaw import AxiomOpenClawManager

async def run_axiom_ceremony():
    manager = AxiomOpenClawManager()
    manager.load_config("axiom_openclaw_config.json")
    
    # Initialize all components
    await manager.initialize_ceremony_master()
    await manager.initialize_health_monitor()
    await manager.initialize_agent_internet()
    
    # Start monitoring in background
    monitor_task = asyncio.create_task(manager.start_monitoring())
    
    try:
        # Start ceremony
        print("🔬 Starting ZK Ceremony...")
        result = await manager.start_ceremony()
        print(f"Ceremony {result['ceremony_id']} started")
        
        # Get status every 10 seconds
        for i in range(10):
            await asyncio.sleep(10)
            status = manager.show_status()
            print(f"Status Update {i+1}: {status}")
        
    finally:
        # Stop monitoring
        await manager.stop_monitoring()
        monitor_task.cancel()

if __name__ == "__main__":
    asyncio.run(run_axiom_ceremony())
```

---

## Deployment on Agent Platforms

### Moltbook
```bash
# Deploy ceremony master agent
moltbook publish --skill ceremony_master \
  --endpoint https://axiom-ceremony.moltbook.ai \
  --auth sk_axiom_ceremony_xxx
```

### Eliza
```bash
# Add agent to Eliza client
eliza add --agent axiom_network_monitor \
  --provider axiom-provider \
  --actions checkNodeHealth,startCeremony
```

### Autonolas
```bash
# Register autonomous service
autonomy publish --service axiom_oracle \
  --token OLAS \
  --stake 5000 \
  --skills ceremony_master,node_health_monitor
```

---

## Monitoring & Debugging

### Check Ceremony Status
```python
history = await ceremony.get_handoff_history()
for handoff in history:
    print(f"  {handoff.miner_name}: {handoff.status}")
```

### Health Metrics Export
```python
await monitor.export_metrics("axiom_metrics_2026-02-04.json")
```

### Agent Deployment Status
```python
for agent_id, agent in manager.agent_internet.agents.items():
    print(f"Agent: {agent.agent_name} ({agent.platform})")
    print(f"  Skills: {', '.join(agent.skills)}")
```

---

## License
Part of the Axiom Protocol - See LICENSE file

## Security Notes
- Miner endpoints must use HTTPS in production
- Hash verification is mandatory for all .zkey files
- Agent platform authentication tokens stored in config (use env vars in prod)
- Monitor alerts can trigger automated responses via webhooks

---

**Questions?** See [CONTRIBUTING.md](../CONTRIBUTING.md) or open an issue.
