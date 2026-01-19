# Qubit Protocol Developer Documentation (Draft)

## Getting Started
- See README for project overview and status
- See TECHNICAL_SPEC.md for architecture and modules
- See ROADMAP.md for priorities

## Building & Running
- Requires Rust 1.70+
- `cargo build` to compile
- `cargo test` to run unit tests
- Use provided scripts for launching nodes (see launch-node.sh)

## Code Structure
- `src/` contains core modules (block, chain, network, etc.)
- `tests/` contains basic unit tests
- `contrib/` contains service scripts

## Adding Features
- Fork the repo and create a new branch
- Add code in appropriate module
- Write tests for new features
- Document changes in IMPLEMENTATION_STATUS.md

## Testing
- Add unit, integration, and adversarial tests as needed

## Contributing
- See CONTRIBUTING.md for guidelines

---
This documentation will be expanded as the project grows.