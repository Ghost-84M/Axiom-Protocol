// src/guardian/safety_manifest.rs
// IMMUTABLE SAFETY MANIFEST - Cannot be modified by AI at runtime
// Foundational invariants that protect the protocol

use crate::error::AxiomError;

/// Sovereign Invariants - Core protocol rules that are HARDCODED and UNMUTABLE
/// All AI decisions must comply with these rules
pub struct SovereignInvariants;

impl SovereignInvariants {
    // ==================== SUPPLY INVARIANTS ====================
    /// Maximum total supply: 124 Million AXM
    pub const MAX_TOTAL_SUPPLY: u64 = 124_000_000_00000000; // 124M with 8 decimals
    
    /// Genesis premine - Start with zero, true mining from genesis
    pub const GENESIS_PREMINE: u64 = 0;
    
    /// Initial block reward: 50 AXM
    pub const INITIAL_BLOCK_REWARD: u64 = 50_00000000; // 50 AXM (8 decimals)
    
    /// Halving interval: 1,240,000 blocks
    pub const HALVING_INTERVAL: u64 = 1_240_000;
    
    // ==================== TEMPORAL INVARIANTS ====================
    /// Target block time: 1,800 seconds (30 minutes)
    pub const TARGET_BLOCK_TIME_SECS: u64 = 1_800;
    
    /// Minimum VDF iterations for security
    pub const MINIMUM_VDF_ITERATIONS: u64 = 1_000_000;
    
    /// Maximum block time deviation: ±300 seconds (±5 minutes)
    pub const MAXIMUM_BLOCK_TIME_DEVIATION_SECS: u64 = 300;
    
    // ==================== AI GOVERNANCE BOUNDS ====================
    /// Maximum difficulty swing: ±5%
    pub const MAX_AI_DIFFICULTY_SWING_PERCENT: f32 = 5.0;
    
    /// Maximum gas price swing: ±10%
    pub const MAX_AI_GAS_SWING_PERCENT: f32 = 10.0;
    
    /// Maximum VDF iterations swing: ±2%
    pub const MAX_AI_VDF_SWING_PERCENT: f32 = 2.0;
    
    /// Upgrade voting period: 48 blocks (24 hours at 30-min blocks)
    pub const UPGRADE_VOTING_PERIOD_BLOCKS: u64 = 48;
    
    // ==================== SECURITY BOUNDS ====================
    /// Minimum peers required for consensus
    pub const MIN_PEERS_FOR_CONSENSUS: usize = 4;
    
    /// Maximum block size: 1 MB
    pub const MAX_BLOCK_SIZE_BYTES: usize = 1_000_000;
    
    /// Minimum transaction fee: 1000 (0.00001 AXM)
    pub const MIN_TRANSACTION_FEE: u64 = 1000;
    
    // ==================== GENESIS CONFIGURATION ====================
    /// Genesis validator count (4 active nodes)
    pub const GENESIS_VALIDATORS: usize = 4;
    
    /// Genesis BFT threshold: 3-of-4 multisig
    pub const GENESIS_BFT_THRESHOLD: usize = 3;
    
    // ==================== CRYPTOGRAPHIC PARAMETERS ====================
    /// Hash output size for Blake3 (bits) - upgradeable from 256 to 512
    pub const BLAKE3_OUTPUT_BITS_LEGACY: usize = 256;
    pub const BLAKE3_OUTPUT_BITS_HYBRID: usize = 384;
    pub const BLAKE3_OUTPUT_BITS_POSTQC: usize = 512;
    
    /// No deprecated algorithms in new transactions
    pub const DEPRECATED_SHA256D: &str = "SHA256d_DISABLED_POST_QUANTUM_ERA";

    // ==================== SUPPLY VERIFICATION ====================
    /// Verify transaction amount doesn't exceed protocol supply cap
    pub fn verify_supply_integrity(current_supply: u64) -> Result<(), AxiomError> {
        if current_supply > Self::MAX_TOTAL_SUPPLY {
            return Err(AxiomError::AIProposalRejected {
                reason: format!(
                    "Supply cap violation: {} > {}",
                    current_supply, Self::MAX_TOTAL_SUPPLY
                ),
            });
        }
        Ok(())
    }

    // ==================== BLOCK REWARD VERIFICATION ====================
    /// Calculate expected reward for given block height
    pub fn calculate_expected_reward(height: u64) -> u64 {
        let era = height / Self::HALVING_INTERVAL;
        let halvings = era.min(63); // Max 63 halvings before reward → 0
        Self::INITIAL_BLOCK_REWARD >> halvings
    }

    /// Verify block reward matches protocol rule
    pub fn verify_block_reward(height: u64, reward: u64) -> Result<(), AxiomError> {
        let expected = Self::calculate_expected_reward(height);
        if reward != expected {
            return Err(AxiomError::InvalidBlockReward {
                expected,
                actual: reward,
            });
        }
        Ok(())
    }

    // ==================== BLOCK TIME VERIFICATION ====================
    /// Verify block time is within acceptable deviation from target
    pub fn verify_block_time(block_time: u64) -> Result<(), AxiomError> {
        let deviation = if block_time > Self::TARGET_BLOCK_TIME_SECS {
            block_time - Self::TARGET_BLOCK_TIME_SECS
        } else {
            Self::TARGET_BLOCK_TIME_SECS - block_time
        };

        if deviation > Self::MAXIMUM_BLOCK_TIME_DEVIATION_SECS {
            return Err(AxiomError::AIProposalRejected {
                reason: format!(
                    "Block time violation: {} secs (target: {} ±{} secs)",
                    block_time,
                    Self::TARGET_BLOCK_TIME_SECS,
                    Self::MAXIMUM_BLOCK_TIME_DEVIATION_SECS
                ),
            });
        }
        Ok(())
    }

    // ==================== AI PROPOSAL VERIFICATION ====================
    /// Verify AI difficulty proposal stays within bounds
    pub fn verify_ai_difficulty_proposal(
        current: u64,
        proposed: u64,
    ) -> Result<(), AxiomError> {
        let ratio = if proposed > current {
            proposed as f64 / current as f64
        } else {
            current as f64 / proposed as f64
        };

        let max_ratio = 1.0 + (Self::MAX_AI_DIFFICULTY_SWING_PERCENT as f64 / 100.0);

        if ratio > max_ratio {
            return Err(AxiomError::AIProposalRejected {
                reason: format!(
                    "Difficulty change exceeds {:.1}% limit: {:.2}% proposed",
                    Self::MAX_AI_DIFFICULTY_SWING_PERCENT,
                    ((ratio - 1.0) * 100.0)
                ),
            });
        }
        Ok(())
    }

    /// Verify AI gas price proposal stays within bounds
    pub fn verify_ai_gas_proposal(
        current: u64,
        proposed: u64,
    ) -> Result<(), AxiomError> {
        let ratio = if proposed > current {
            proposed as f64 / current as f64
        } else {
            current as f64 / proposed as f64
        };

        let max_ratio = 1.0 + (Self::MAX_AI_GAS_SWING_PERCENT as f64 / 100.0);

        if ratio > max_ratio {
            return Err(AxiomError::AIProposalRejected {
                reason: format!(
                    "Gas change exceeds {:.1}% limit: {:.2}% proposed",
                    Self::MAX_AI_GAS_SWING_PERCENT,
                    ((ratio - 1.0) * 100.0)
                ),
            });
        }
        Ok(())
    }

    /// Verify AI VDF iterations proposal stays within bounds AND above minimum
    pub fn verify_ai_vdf_proposal(
        current: u64,
        proposed: u64,
    ) -> Result<(), AxiomError> {
        // Check minimum threshold
        if proposed < Self::MINIMUM_VDF_ITERATIONS {
            return Err(AxiomError::AIProposalRejected {
                reason: format!(
                    "VDF iterations {} below minimum {}",
                    proposed,
                    Self::MINIMUM_VDF_ITERATIONS
                ),
            });
        }

        // Check percentage bounds
        let ratio = if proposed > current {
            proposed as f64 / current as f64
        } else {
            current as f64 / proposed as f64
        };

        let max_ratio = 1.0 + (Self::MAX_AI_VDF_SWING_PERCENT as f64 / 100.0);

        if ratio > max_ratio {
            return Err(AxiomError::AIProposalRejected {
                reason: format!(
                    "VDF change exceeds {:.1}% limit: {:.2}% proposed",
                    Self::MAX_AI_VDF_SWING_PERCENT,
                    ((ratio - 1.0) * 100.0)
                ),
            });
        }
        Ok(())
    }

    /// Verify minimum transaction fee requirement
    pub fn verify_transaction_fee(fee: u64) -> Result<(), AxiomError> {
        if fee < Self::MIN_TRANSACTION_FEE {
            return Err(AxiomError::FeeTooLow {
                min: Self::MIN_TRANSACTION_FEE,
                actual: fee,
            });
        }
        Ok(())
    }

    /// Verify block size doesn't exceed maximum
    pub fn verify_block_size(size: usize) -> Result<(), AxiomError> {
        if size > Self::MAX_BLOCK_SIZE_BYTES {
            return Err(AxiomError::InvalidBlock(
                format!(
                    "Block too large: {} bytes (max: {})",
                    size, Self::MAX_BLOCK_SIZE_BYTES
                )
            ));
        }
        Ok(())
    }

    /// Get total supply after N blocks
    pub fn calculate_supply_at_height(height: u64) -> u64 {
        let mut total = 0u64;
        let mut era = 0u64;

        while era * Self::HALVING_INTERVAL < height {
            let blocks_in_era = if (era + 1) * Self::HALVING_INTERVAL <= height {
                Self::HALVING_INTERVAL
            } else {
                height - (era * Self::HALVING_INTERVAL)
            };

            let reward = Self::INITIAL_BLOCK_REWARD >> era.min(63);
            total = total.saturating_add(blocks_in_era.saturating_mul(reward));

            era += 1;
        }

        total.min(Self::MAX_TOTAL_SUPPLY)
    }
}

// ==================== TESTS ====================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supply_cap_enforcement() {
        assert!(SovereignInvariants::verify_supply_integrity(100_000_000_00000000).is_ok());
        assert!(SovereignInvariants::verify_supply_integrity(124_000_000_00000000).is_ok());
        assert!(SovereignInvariants::verify_supply_integrity(124_000_001_00000000).is_err());
    }

    #[test]
    fn test_block_reward_halving() {
        // Era 0: 50 AXM
        assert_eq!(SovereignInvariants::calculate_expected_reward(0), 50_00000000);
        
        // Era 1 (after 1.24M blocks): 25 AXM
        assert_eq!(
            SovereignInvariants::calculate_expected_reward(1_240_000),
            25_00000000
        );
        
        // Era 2 (after 2.48M blocks): 12.5 AXM
        assert_eq!(
            SovereignInvariants::calculate_expected_reward(2_480_000),
            12_50000000
        );
    }

    #[test]
    fn test_block_reward_verification() {
        assert!(SovereignInvariants::verify_block_reward(0, 50_00000000).is_ok());
        assert!(SovereignInvariants::verify_block_reward(0, 51_00000000).is_err());
        assert!(SovereignInvariants::verify_block_reward(1_240_000, 25_00000000).is_ok());
    }

    #[test]
    fn test_ai_difficulty_bounds() {
        // 5% increase is OK
        assert!(SovereignInvariants::verify_ai_difficulty_proposal(1_000_000, 1_050_000).is_ok());
        
        // 6% increase is NOT OK
        assert!(SovereignInvariants::verify_ai_difficulty_proposal(1_000_000, 1_060_000).is_err());
        
        // ~5% decrease: 1_000_000 / 952_381 ≈ 1.05 (at boundary, should pass)
        assert!(SovereignInvariants::verify_ai_difficulty_proposal(1_000_000, 952_381).is_ok());
    }

    #[test]
    fn test_vdf_minimum_enforcement() {
        // Above minimum with <2% change: OK
        assert!(SovereignInvariants::verify_ai_vdf_proposal(1_000_000, 1_020_000).is_ok());
        
        // Below minimum: NOT OK
        assert!(SovereignInvariants::verify_ai_vdf_proposal(1_000_000, 999_999).is_err());
        
        // Way too low: NOT OK
        assert!(SovereignInvariants::verify_ai_vdf_proposal(1_000_000, 500_000).is_err());
    }

    #[test]
    fn test_block_time_verification() {
        // Exactly at target: OK
        assert!(SovereignInvariants::verify_block_time(1_800).is_ok());
        
        // Within ±5 min (±300s): OK
        assert!(SovereignInvariants::verify_block_time(1_500).is_ok());
        assert!(SovereignInvariants::verify_block_time(2_100).is_ok());
        
        // Beyond ±5 min: NOT OK
        assert!(SovereignInvariants::verify_block_time(1_499).is_err());
        assert!(SovereignInvariants::verify_block_time(2_101).is_err());
    }

    #[test]
    fn test_supply_calculation() {
        // At block 0: exactly 50 AXM
        assert_eq!(
            SovereignInvariants::calculate_supply_at_height(1),
            50_00000000
        );

        // At halving boundary: still 50M blocks × 50 AXM = 2.5M total
        let supply_at_halving = SovereignInvariants::calculate_supply_at_height(1_240_000);
        assert!(supply_at_halving > 0);
        assert!(supply_at_halving <= SovereignInvariants::MAX_TOTAL_SUPPLY);
    }
}
