# Qubit Protocol Economics & Tokenomics (Draft)

## Total Supply
- Fixed supply: 84,000,000 coins
- No inflation or deflation mechanisms

## Block Rewards
- Halving schedule: Formula implemented, rewards halve every 2,100,000 blocks
- No priority fee mechanism (EIP-1559 style, planned)
- Minimum fee enforced: 0.01 QBT (1,000,000 units)

## Transaction Fees
- Dynamic fee market logic (fees can be adjusted based on network demand)
- Minimum fee required for all transactions (prevents spam)

## Distribution
- Fair initial distribution: No premine, no central treasury, no burn mechanism
- All coins are mined via consensus, rewards are distributed to miners

## Economic Security
- Minimum fee and dynamic fee market help prevent spam and incentivize miners
- Anti-centralization: Mining pool detection and reward capping implemented (max 10 consecutive blocks per address)

---
This document will be updated as economic mechanisms are designed. See economics.rs and chain.rs for minimum fee, reward logic, and anti-centralization mechanisms.