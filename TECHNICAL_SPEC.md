# Qubit Protocol - Technical Specification (Draft)

## 1. Overview
Qubit Protocol is an experimental blockchain project focused on ZK-SNARK privacy, VDF-based consensus, and P2P networking in Rust. It is not production-ready and is intended for educational and research purposes.

## 2. Architecture
- **Language:** Rust
- **Database:** sled (embedded)
- **Networking:** libp2p (mDNS only)
- **Consensus:** VDF (incomplete), PoW hybrid (planned)
- **Privacy:** ZK-SNARKs (circuit file present, not functional)
- **AI/Neural Guardian:** Heuristic-based anomaly detection (no ML/AI)

## 3. Core Modules
- **Block:** Basic block structure, header, and serialization
- **Chain:** Chain management, block addition, validation
- **State:** State storage, sled-backed
- **Network:** P2P messaging, peer management
- **Transaction:** Basic transaction format, validation
- **Wallet:** Key management (basic, not secure)

## 4. Consensus
- **VDF:** Module present, not fully implemented
- **Block Time:** Target 1 hour (not enforced/tested)
- **Difficulty:** Static (no adjustment)
- **Fork Choice:** Not specified

## 5. Privacy
- **ZK-SNARKs:** Circuit file exists, no trusted setup, no proof verification
- **Curve:** Not documented
- **Privacy Guarantees:** Not specified

## 6. Networking
- **Peer Discovery:** mDNS only
- **Peer Limits:** Not enforced
- **Security:** No Sybil/eclipse/DDOS protection

## 7. Economics
- **Tokenomics:** Fixed supply (84M), no fee market, no inflation/deflation analysis
- **Distribution:** No genesis plan, premine, or treasury

## 8. Testing
- **Unit Tests:** Basic, 8/8 passing
- **Integration/Adversarial:** Not present
- **Benchmarks/Fuzzing:** Not present

## 9. Documentation
- **README:** Honest status, experimental
- **Spec:** This document (draft)
- **Whitepaper:** Not present

## 10. Roadmap
See ROADMAP.md for planned features and priorities.

---
This spec will be updated as development progresses.