// Production modules
pub mod error;
pub mod config;
pub mod mempool;

// Core modules
pub mod zk;
pub mod consensus; // VDF consensus implementation
pub mod ai; // AI Oracle network
// Re-export modules and wallet so they can be used by bin crates
pub mod transaction;
pub mod main_helper;
pub mod block;
pub mod genesis;
pub mod chain;
pub mod state;
pub mod economics;
pub mod wallet;
pub mod vdf;
pub mod ai_engine;
pub mod bridge;
pub mod time;
pub mod storage;
pub mod network;
pub mod neural_guardian; // NEW: AI-powered security with federated learning
pub use main_helper::Wallet;
pub use block::Block;

// Re-export production types
pub use error::{AxiomError, Result};
pub use config::AxiomConfig;
