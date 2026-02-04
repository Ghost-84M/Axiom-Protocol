#!/usr/bin/env python3
"""
Quick start script for Axiom OpenClaw integration
Run this to launch the interactive manager
"""

import asyncio
import sys
import os

# Add openclaw to path
sys.path.insert(0, os.path.dirname(__file__))

from manager import AxiomOpenClawManager


async def main():
    """Launch the manager"""
    manager = AxiomOpenClawManager()
    
    # Load config (will create default if not exists)
    config_path = os.path.join(
        os.path.dirname(__file__), 
        "axiom_openclaw_config.json"
    )
    manager.load_config(config_path)
    
    # Initialize all components
    print("\n" + "=" * 70)
    print("🚀 AXIOM PROTOCOL - OPENCLAW INTEGRATION")
    print("=" * 70)
    print("\n⚙️  Initializing components...\n")
    
    await manager.initialize_ceremony_master()
    await manager.initialize_health_monitor()
    await manager.initialize_agent_internet()
    
    print("\n" + "=" * 70)
    print("✅ ALL COMPONENTS READY")
    print("=" * 70)
    
    # Start interactive console
    await manager.interactive_console()


if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        print("\n\n🛑 Shutdown complete")
        sys.exit(0)
    except Exception as e:
        print(f"\n❌ Error: {e}")
        sys.exit(1)
