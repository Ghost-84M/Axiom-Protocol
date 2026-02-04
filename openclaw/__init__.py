"""
Axiom Protocol OpenClaw Integration Module

Three integrated components for agent-based automation:

1. **Ceremony Master** - Automates Phase 2 ZK-Ceremony coordination
   - Initiates miners in sequence
   - Verifies zkey contributions
   - Hands off between participants
   - Complete automation without human intervention

2. **Node Health Monitor** - Real-time network surveillance
   - Tracks peer connectivity
   - Measures latency and throughput
   - Detects anomalies and issues
   - Generates alerts and reports

3. **Agent Internet Integration** - Multi-platform deployment
   - Moltbook - Decentralized AI agent marketplace
   - Eliza - Modular agent framework
   - Autonolas - Autonomous service network
   - Manages skill deployment and agent coordination
"""

from ceremony_master import CeremonyMaster, CEREMONY_MASTER_SKILL
from node_health_monitor import NodeHealthMonitor, NODE_HEALTH_MONITOR_SKILL
from agent_internet import (
    AgentInternetIntegration,
    create_axiom_agents,
    AGENT_INTERNET_INTEGRATION,
)
from manager import AxiomOpenClawManager

__all__ = [
    "CeremonyMaster",
    "CEREMONY_MASTER_SKILL",
    "NodeHealthMonitor",
    "NODE_HEALTH_MONITOR_SKILL",
    "AgentInternetIntegration",
    "create_axiom_agents",
    "AGENT_INTERNET_INTEGRATION",
    "AxiomOpenClawManager",
]
