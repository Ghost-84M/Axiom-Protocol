# Qubit Protocol Network Protocol (Draft)

## Peer Discovery
- mDNS for local network
- DHT and bootstrap node support for global peer discovery (production-grade)

## Peer Management
- Peer connection limits enforced (default: 32)
- Basic peer banning mechanism (rate limiting, ban list)
- No geographic diversity requirements

## Message Formats
- Basic block and transaction relay
- No formal specification of message types

## Security
- Peer authentication planned
- Sybil/eclipse/DDOS protection groundwork added (rate limiting, ban logic)

## Synchronization
- Sync protocol for new nodes via request-response and chain broadcast
- No state snapshots or fast sync (planned)

---
This document will be expanded as networking features are added. See network.rs for implementation details on connection limits, ban logic, and DHT/bootstrap support.