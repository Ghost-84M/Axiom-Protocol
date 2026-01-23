/// ECONOMICS — CONSENSUS MONETARY POLICY
/// Fixed total supply (forever)

pub const MAX_SUPPLY: u64 = 8_400_000_000_000_000; // 84M AXM, 8 decimals (84,000,000 * 10^8)

/// Initial block reward (50 AXM, 8 decimals)
pub const INITIAL_REWARD: u64 = 5_000_000_000;

/// Blocks per halving (Litecoin-like: 840,000 blocks)
/// Math: 50 AXM × 840,000 blocks × 2 (geometric series) = 84,000,000 AXM ✓
pub const HALVING_INTERVAL: u64 = 840_000;

/// Calculate block reward based on height
pub fn block_reward(block_height: u64, already_issued: u64) -> u64 {
    let halvings = block_height / HALVING_INTERVAL;

    let reward = INITIAL_REWARD >> halvings;

    if reward == 0 {
        return 0;
    }

    if already_issued + reward > MAX_SUPPLY {
        MAX_SUPPLY - already_issued
    } else {
        reward
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supply_math_correct() {
        // Verify total supply calculation
        let mut total = 0u64;
        let mut height = 0u64;
        let mut reward = INITIAL_REWARD;
        
        while reward > 0 {
            let blocks_at_this_reward = HALVING_INTERVAL;
            total += blocks_at_this_reward * reward;
            height += blocks_at_this_reward;
            reward = reward >> 1; // Halve
        }
        
        // Should be very close to 84M (within 0.01% due to integer rounding)
        // Actual: 83,999,999.91 AXM ≈ 84M AXM
        let difference = if total > MAX_SUPPLY {
            total - MAX_SUPPLY
        } else {
            MAX_SUPPLY - total
        };
        let tolerance = MAX_SUPPLY / 10000; // 0.01% tolerance
        assert!(difference < tolerance, 
            "Supply {} differs from MAX_SUPPLY {} by more than 0.01%", 
            total, MAX_SUPPLY);
        
        println!("✓ Total supply: {} AXM (difference from 84M: {} smallest units)", 
            total as f64 / 1e8, difference);
    }
    
    #[test]
    fn test_halving_schedule() {
        assert_eq!(block_reward(0, 0), 5_000_000_000); // Block 0: 50 AXM
        assert_eq!(block_reward(840_000, 0), 2_500_000_000); // First halving: 25 AXM
        assert_eq!(block_reward(1_680_000, 0), 1_250_000_000); // Second: 12.5 AXM
        assert_eq!(block_reward(2_520_000, 0), 625_000_000); // Third: 6.25 AXM
    }
    
    #[test]
    fn test_supply_cap_enforced() {
        // Test that we never exceed MAX_SUPPLY
        let near_max = MAX_SUPPLY - 100;
        assert_eq!(block_reward(0, near_max), 100); // Should return remaining only
        assert_eq!(block_reward(0, MAX_SUPPLY), 0); // Should return 0 when at cap
    }
}
