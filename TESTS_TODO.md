# Qubit Protocol - Testing TODOs

## Core Tests Needed
- Double spend prevention
- 51% attack resistance
- Network partition recovery
- Block propagation under load
- Byzantine peer isolation
- Chain reorganization limits

## Load & Performance
- TPS (transactions per second) benchmarks
- Block propagation time measurements
- Memory usage profiling
- Network bandwidth analysis
- State size growth projections

## Fuzz & Property-Based Testing
- Fuzzing for consensus-critical code
- Property-based tests for state transitions

---
Add new tests in `tests/integration_tests.rs` and update this file as coverage improves.