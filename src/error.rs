// src/error.rs - AXIOM Protocol Complete Error Handling
// Production-ready error types for mainnet deployment

use thiserror::Error;

/// Main error type for AXIOM Protocol
#[derive(Error, Debug)]
pub enum AxiomError {
    // ==================== TRANSACTION ERRORS ====================
    #[error("Insufficient balance: account has {available} AXM, needs {required} AXM")]
    InsufficientBalance {
        available: u64,
        required: u64,
    },
    
    #[error("Invalid transaction nonce: expected {expected}, got {actual}")]
    InvalidNonce {
        expected: u64,
        actual: u64,
    },
    
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),
    
    #[error("Transaction amount cannot be zero")]
    ZeroAmount,
    
    #[error("Transaction fee too low: minimum {min} AXM, got {actual} AXM")]
    FeeTooLow {
        min: u64,
        actual: u64,
    },
    
    #[error("Transaction already in mempool")]
    DuplicateTransaction,
    
    #[error("Transaction too large: {size} bytes (max: {max} bytes)")]
    TransactionTooLarge {
        size: usize,
        max: usize,
    },
    
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
    
    #[error("Transaction expired: submitted at {submitted}, current time {current}")]
    TransactionExpired {
        submitted: u64,
        current: u64,
    },
    
    #[error("Nullifier already used (double-spend attempt)")]
    NullifierUsed,
    
    // ==================== BLOCK ERRORS ====================
    #[error("Invalid block: {0}")]
    InvalidBlock(String),
    
    #[error("Block parent mismatch: expected {expected}, got {actual}")]
    InvalidParent {
        expected: String,
        actual: String,
    },
    
    #[error("Invalid block height: expected {expected}, got {actual}")]
    InvalidBlockHeight {
        expected: u64,
        actual: u64,
    },
    
    #[error("Block timestamp invalid: {timestamp} (current: {current})")]
    InvalidTimestamp {
        timestamp: u64,
        current: u64,
    },
    
    #[error("Block too large: {size} bytes (max: {max} bytes)")]
    BlockTooLarge {
        size: usize,
        max: usize,
    },
    
    #[error("Invalid block reward: expected {expected} AXM, got {actual} AXM")]
    InvalidBlockReward {
        expected: u64,
        actual: u64,
    },
    
    #[error("Block not found: {0}")]
    BlockNotFound(String),
    
    #[error("Genesis block already exists")]
    GenesisExists,
    
    // ==================== CONSENSUS ERRORS ====================
    #[error("VDF verification failed: {0}")]
    VDFVerificationFailed(String),
    
    #[error("VDF computation failed: {0}")]
    VDFComputationFailed(String),
    
    #[error("Proof of Work verification failed: hash {hash} doesn't meet difficulty {difficulty}")]
    PoWVerificationFailed {
        hash: String,
        difficulty: u64,
    },
    
    #[error("Chain reorganization required: depth {depth} blocks")]
    ChainReorgRequired {
        depth: u64,
    },
    
    #[error("Fork detected at height {height}")]
    ForkDetected {
        height: u64,
    },
    
    #[error("Consensus failure: {0}")]
    ConsensusFailed(String),
    
    // ==================== CRYPTOGRAPHY ERRORS ====================
    #[error("ZK proof generation failed: {0}")]
    ProofGenerationFailed(String),
    
    #[error("ZK proof verification failed: {0}")]
    ProofVerificationFailed(String),
    
    #[error("Invalid ZK circuit parameters: {0}")]
    InvalidCircuitParams(String),
    
    #[error("Cryptographic operation failed: {0}")]
    CryptoError(String),
    
    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),
    
    #[error("Invalid private key format")]
    InvalidPrivateKey,
    
    #[error("Key derivation failed: {0}")]
    KeyDerivationFailed(String),
    
    // ==================== NETWORK ERRORS ====================
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Peer connection failed: {peer_id}")]
    PeerConnectionFailed {
        peer_id: String,
    },
    
    #[error("Maximum peers reached: {max}")]
    MaxPeersReached {
        max: usize,
    },
    
    #[error("Peer banned: {peer_id} (reason: {reason})")]
    PeerBanned {
        peer_id: String,
        reason: String,
    },
    
    #[error("Network timeout after {seconds} seconds")]
    NetworkTimeout {
        seconds: u64,
    },
    
    #[error("Gossip protocol error: {0}")]
    GossipError(String),
    
    #[error("P2P protocol error: {0}")]
    P2PError(String),
    
    // ==================== STORAGE ERRORS ====================
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("State corruption detected: {0}")]
    StateCorruption(String),
    
    #[error("Account not found: {0}")]
    AccountNotFound(String),
    
    #[error("Storage initialization failed: {0}")]
    StorageInitFailed(String),
    
    #[error("Disk full: {available} bytes available, need {required} bytes")]
    DiskFull {
        available: u64,
        required: u64,
    },
    
    #[error("Database migration failed: {0}")]
    MigrationFailed(String),
    
    // ==================== AI/ORACLE ERRORS ====================
    #[error("Neural Guardian threat detected: {threat_type} (confidence: {confidence})")]
    ThreatDetected {
        threat_type: String,
        confidence: f32,
    },
    
    #[error("AI proposal rejected: {reason}")]
    AIProposalRejected {
        reason: String,
    },
    
    #[error("Oracle consensus failed: got {responses} responses, need {required}")]
    OracleConsensusFailed {
        responses: usize,
        required: usize,
    },
    
    #[error("AI model error: {0}")]
    AIModelError(String),
    
    #[error("Oracle stake insufficient: have {have} AXM, need {need} AXM")]
    InsufficientStake {
        have: u64,
        need: u64,
    },
    
    #[error("Supply cap violation: current {current} exceeds max {max}")]
    SupplyCapViolation {
        current: u64,
        max: u64,
    },
    
    #[error("Block time violation: actual {actual} target {target} max_deviation {max_deviation}")]
    BlockTimingViolation {
        actual: u64,
        target: u64,
        max_deviation: u64,
    },
    
    // ==================== CONFIGURATION ERRORS ====================
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    
    #[error("Missing required parameter: {0}")]
    MissingParameter(String),
    
    #[error("Configuration file not found: {0}")]
    ConfigNotFound(String),
    
    #[error("Configuration parse error: {0}")]
    ConfigParseError(String),
    
    // ==================== WALLET ERRORS ====================
    #[error("Wallet locked: unlock with password")]
    WalletLocked,
    
    #[error("Invalid password")]
    InvalidPassword,
    
    #[error("Wallet file not found: {0}")]
    WalletNotFound(String),
    
    #[error("Wallet already exists: {0}")]
    WalletExists(String),
    
    #[error("Keystore error: {0}")]
    KeystoreError(String),
    
    // ==================== RPC ERRORS ====================
    #[error("RPC error: {0}")]
    RpcError(String),
    
    #[error("Invalid RPC request: {0}")]
    InvalidRpcRequest(String),
    
    #[error("RPC method not found: {0}")]
    RpcMethodNotFound(String),
    
    #[error("RPC timeout")]
    RpcTimeout,
    
    // ==================== SYSTEM ERRORS ====================
    #[error("I/O error: {0}")]
    IoError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    
    #[error("Thread error: {0}")]
    ThreadError(String),
    
    #[error("System resource exhausted: {0}")]
    ResourceExhausted(String),
    
    #[error("Emergency shutdown initiated: {reason}")]
    EmergencyShutdown {
        reason: String,
    },
}

/// Result type alias for AXIOM Protocol
pub type Result<T> = std::result::Result<T, AxiomError>;

// ==================== ERROR CONVERSIONS ====================

impl From<std::io::Error> for AxiomError {
    fn from(err: std::io::Error) -> Self {
        AxiomError::IoError(err.to_string())
    }
}

impl From<sled::Error> for AxiomError {
    fn from(err: sled::Error) -> Self {
        AxiomError::DatabaseError(err.to_string())
    }
}

impl From<bincode::Error> for AxiomError {
    fn from(err: bincode::Error) -> Self {
        AxiomError::SerializationError(err.to_string())
    }
}

impl From<toml::de::Error> for AxiomError {
    fn from(err: toml::de::Error) -> Self {
        AxiomError::ConfigParseError(err.to_string())
    }
}

// ==================== ERROR SEVERITY ====================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// Critical error - node should halt
    Critical,
    /// Major error - feature unavailable but node can continue
    Major,
    /// Minor error - can be handled gracefully
    Minor,
}

impl AxiomError {
    /// Get severity level of error
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            // Critical errors - halt node
            AxiomError::StateCorruption(_) |
            AxiomError::GenesisExists |
            AxiomError::EmergencyShutdown { .. } |
            AxiomError::StorageInitFailed(_) |
            AxiomError::DiskFull { .. } => ErrorSeverity::Critical,
            
            // Major errors - feature unavailable
            AxiomError::DatabaseError(_) |
            AxiomError::NetworkError(_) |
            AxiomError::ConsensusFailed(_) |
            AxiomError::ChainReorgRequired { .. } => ErrorSeverity::Major,
            
            // Minor errors - can be handled
            _ => ErrorSeverity::Minor,
        }
    }
    
    /// Check if error is critical (should halt node)
    pub fn is_critical(&self) -> bool {
        self.severity() == ErrorSeverity::Critical
    }
    
    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        !self.is_critical()
    }
}

/// Install panic hook for graceful shutdown
pub fn install_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("🚨 PANIC: {}", panic_info);
        eprintln!("Node will shut down to prevent state corruption");
        std::process::exit(1);
    }));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_severity() {
        assert_eq!(
            AxiomError::StateCorruption("test".into()).severity(),
            ErrorSeverity::Critical
        );
        
        assert_eq!(
            AxiomError::InvalidNonce { expected: 1, actual: 2 }.severity(),
            ErrorSeverity::Minor
        );
    }
    
    #[test]
    fn test_error_conversions() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
        let axiom_err: AxiomError = io_err.into();
        
        assert!(matches!(axiom_err, AxiomError::IoError(_)));
    }
}
