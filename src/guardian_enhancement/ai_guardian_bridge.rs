// src/guardian_enhancement/ai_guardian_bridge.rs
// Guardian-AI Bridge - Integrates AI decisions with Governor Safety Manifest
// CRITICAL: All AI decisions require Guardian verification

use crate::guardian::SovereignInvariants;
use crate::ai_core::{
    MultiLayerSecurityEngine, ThreatAssessment, SecurityAction, RiskLevel, TransactionRiskProfile,
};
use crate::error::AxiomError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use parking_lot::RwLock;

/// Guardian-enforced AI governance
pub struct AIGuardianBridge {
    security_engine: Arc<MultiLayerSecurityEngine>,
    guardian_state: Arc<RwLock<GuardianState>>,
    consensus_ai: Arc<RwLock<ConsensusAIController>>,
    emergency_circuit_breaker: Arc<RwLock<CircuitBreaker>>,
}

#[derive(Debug, Clone)]
struct GuardianState {
    ai_enabled: bool,
    auto_pilot_mode: bool,
    manual_override_active: bool,
    total_ai_decisions: u64,
    guardian_vetoes: u64,
    last_veto_reason: Option<String>,
}

/// AI-driven consensus optimizer with Guardian bounds
pub struct ConsensusAIController {
    current_difficulty: u64,
    current_vdf_iterations: u64,
    current_min_gas: u64,
    
    // PID controllers for smooth adjustments
    difficulty_pid: PIDController,
    gas_pid: PIDController,
    vdf_pid: PIDController,
    
    // Historical data (last 1000 blocks)
    block_time_history: Vec<u64>,
    hashrate_history: Vec<f64>,
    mempool_history: Vec<usize>,
    
    // AI learning state
    optimization_history: Vec<OptimizationRecord>,
}

#[derive(Debug, Clone)]
struct PIDController {
    kp: f64,
    ki: f64,
    kd: f64,
    integral: f64,
    previous_error: f64,
    output_min: f64,
    output_max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OptimizationRecord {
    timestamp: u64,
    block_height: u64,
    parameter: String,
    old_value: u64,
    new_value: u64,
    predicted_improvement: f64,
    actual_improvement: f64,
    guardian_approved: bool,
}

#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    is_active: bool,
    activation_block: Option<u64>,
    reason: Option<String>,
    auto_recovery_block: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusOptimizationProposal {
    pub proposal_id: String,
    pub block_height: u64,
    pub timestamp: u64,
    
    // Difficulty adjustment
    pub current_difficulty: u64,
    pub proposed_difficulty: u64,
    pub difficulty_change_percent: f64,
    
    // VDF adjustment
    pub current_vdf: u64,
    pub proposed_vdf: u64,
    pub vdf_change_percent: f64,
    
    // Gas price adjustment
    pub current_min_gas: u64,
    pub proposed_min_gas: u64,
    pub gas_change_percent: f64,
    
    // Metrics
    pub avg_block_time_last_144: f64,
    pub hashrate_trend: f64,
    pub mempool_congestion: f64,
    pub network_health_score: f64,
    
    // Confidence and status
    pub ai_confidence: f64,
    pub expected_improvement: f64,
    pub guardian_pre_approved: bool,
    pub requires_voting: bool,
}

#[derive(Debug, Clone)]
pub struct BlockMetrics {
    pub height: u64,
    pub timestamp: u64,
    pub block_time: u64,
    pub difficulty: u64,
    pub vdf_iterations: u64,
    pub transaction_count: usize,
    pub total_fees: u64,
    pub hashrate_estimate: f64,
}

impl AIGuardianBridge {
    pub fn new(security_engine: Arc<MultiLayerSecurityEngine>) -> Self {
        Self {
            security_engine,
            guardian_state: Arc::new(RwLock::new(GuardianState {
                ai_enabled: true,
                auto_pilot_mode: false,
                manual_override_active: false,
                total_ai_decisions: 0,
                guardian_vetoes: 0,
                last_veto_reason: None,
            })),
            consensus_ai: Arc::new(RwLock::new(ConsensusAIController::new())),
            emergency_circuit_breaker: Arc::new(RwLock::new(CircuitBreaker {
                is_active: false,
                activation_block: None,
                reason: None,
                auto_recovery_block: None,
            })),
        }
    }

    /// Validate transaction with AI + Guardian oversight
    pub fn validate_transaction_with_guardian(
        &self,
        profile: TransactionRiskProfile,
        current_block: u64,
    ) -> Result<GuardianDecision, AxiomError> {
        // Check circuit breaker
        let breaker = self.emergency_circuit_breaker.read();
        if breaker.is_active {
            return Err(AxiomError::AIProposalRejected {
                reason: format!(
                    "Emergency circuit breaker active: {}",
                    breaker.reason.as_ref().unwrap_or(&"Unknown".to_string())
                ),
            });
        }
        drop(breaker);

        // Get AI threat assessment
        let threat_assessment = self.security_engine.assess_transaction_threat(&profile, current_block)?;

        // Guardian verification of AI decision
        let guardian_decision = self.guardian_verify_ai_decision(&threat_assessment, &profile, current_block)?;

        // Update state
        let mut state = self.guardian_state.write();
        state.total_ai_decisions += 1;

        if !guardian_decision.approved {
            state.guardian_vetoes += 1;
            state.last_veto_reason = Some(
                guardian_decision.veto_reason.clone().unwrap_or_default(),
            );
        }

        Ok(guardian_decision)
    }

    /// Guardian verification layer - CANNOT BE BYPASSED
    fn guardian_verify_ai_decision(
        &self,
        ai_assessment: &ThreatAssessment,
        profile: &TransactionRiskProfile,
        _current_block: u64,
    ) -> Result<GuardianDecision, AxiomError> {
        // Rule 1: Verify transaction doesn't exceed supply
        SovereignInvariants::verify_supply_integrity(profile.amount)?;

        // Rule 2: Verify minimum fee
        if profile.gas_price < SovereignInvariants::MIN_TRANSACTION_FEE {
            return Ok(GuardianDecision {
                approved: false,
                veto_reason: Some(format!(
                    "Transaction fee {} below minimum {}",
                    profile.gas_price,
                    SovereignInvariants::MIN_TRANSACTION_FEE
                )),
                action: GuardianAction::Reject,
                threat_assessment: ai_assessment.clone(),
            });
        }

        // Rule 3: Check if AI wants to escalate to Guardian
        if ai_assessment.guardian_override_required {
            log::warn!("🛡️  Guardian override required - AI threat score: {:.2}", ai_assessment.threat_score);

            let state = self.guardian_state.read();
            if state.auto_pilot_mode && matches!(ai_assessment.risk_level, RiskLevel::Catastrophic) {
                return Ok(GuardianDecision {
                    approved: false,
                    veto_reason: Some(format!(
                        "Auto-pilot rejection: Catastrophic threat (score: {:.2})",
                        ai_assessment.threat_score
                    )),
                    action: GuardianAction::AutoReject,
                    threat_assessment: ai_assessment.clone(),
                });
            }
        }

        // Rule 4: Apply AI's recommended action with Guardian bounds
        let action = match &ai_assessment.recommended_action {
            SecurityAction::Accept => GuardianAction::Accept,
            SecurityAction::AcceptWithMonitoring => GuardianAction::AcceptMonitored,
            SecurityAction::Quarantine { duration_blocks } => {
                let max_duration = 1440;
                let safe_duration = (*duration_blocks).min(max_duration);
                GuardianAction::Quarantine {
                    duration_blocks: safe_duration,
                }
            }
            SecurityAction::Reject { reason: _ } => GuardianAction::Reject,
            SecurityAction::EscalateToGuardian { threat_level } => {
                GuardianAction::RequireManualReview {
                    threat_level: *threat_level,
                }
            }
            SecurityAction::HaltChain { emergency_level } => {
                if *emergency_level >= 9 {
                    self.activate_circuit_breaker(
                        0,
                        "AI detected critical chain-level threat".to_string(),
                    )?;
                    GuardianAction::ChainHalt
                } else {
                    GuardianAction::RequireManualReview {
                        threat_level: RiskLevel::Critical,
                    }
                }
            }
        };

        Ok(GuardianDecision {
            approved: !matches!(
                action,
                GuardianAction::Reject | GuardianAction::AutoReject | GuardianAction::ChainHalt
            ),
            veto_reason: None,
            action,
            threat_assessment: ai_assessment.clone(),
        })
    }

    /// Generate consensus optimization proposal
    pub fn generate_consensus_optimization(
        &self,
        current_block: u64,
        recent_blocks: &[BlockMetrics],
    ) -> Result<ConsensusOptimizationProposal, AxiomError> {
        if recent_blocks.len() < 144 {
            return Err(AxiomError::AIProposalRejected {
                reason: "Insufficient block history for optimization".to_string(),
            });
        }

        let mut consensus = self.consensus_ai.write();
        consensus.update_metrics(recent_blocks)?;

        // Calculate optimal parameters
        let difficulty_proposal = consensus.calculate_difficulty_adjustment()?;
        let vdf_proposal = consensus.calculate_vdf_adjustment()?;
        let gas_proposal = consensus.calculate_gas_adjustment()?;

        // Guardian pre-validation
        SovereignInvariants::verify_ai_difficulty_proposal(consensus.current_difficulty, difficulty_proposal)?;
        SovereignInvariants::verify_ai_vdf_proposal(consensus.current_vdf_iterations, vdf_proposal)?;
        SovereignInvariants::verify_ai_gas_proposal(consensus.current_min_gas, gas_proposal)?;

        // Calculate metrics
        let avg_block_time = recent_blocks.iter().map(|b| b.block_time).sum::<u64>() as f64
            / recent_blocks.len() as f64;

        let hashrate_trend = consensus.calculate_hashrate_trend()?;
        let mempool_congestion = consensus.calculate_mempool_congestion()?;
        let network_health = consensus.calculate_network_health_score()?;

        let proposal = ConsensusOptimizationProposal {
            proposal_id: format!("ai_consensus_{}", current_block),
            block_height: current_block,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),

            current_difficulty: consensus.current_difficulty,
            proposed_difficulty: difficulty_proposal,
            difficulty_change_percent: Self::calculate_change_percent(
                consensus.current_difficulty,
                difficulty_proposal,
            ),

            current_vdf: consensus.current_vdf_iterations,
            proposed_vdf: vdf_proposal,
            vdf_change_percent: Self::calculate_change_percent(
                consensus.current_vdf_iterations,
                vdf_proposal,
            ),

            current_min_gas: consensus.current_min_gas,
            proposed_min_gas: gas_proposal,
            gas_change_percent: Self::calculate_change_percent(consensus.current_min_gas, gas_proposal),

            avg_block_time_last_144: avg_block_time,
            hashrate_trend,
            mempool_congestion,
            network_health_score: network_health,

            ai_confidence: consensus.calculate_confidence()?,
            expected_improvement: consensus.calculate_expected_improvement()?,

            guardian_pre_approved: true,
            requires_voting: false,
        };

        Ok(proposal)
    }

    /// Apply consensus optimization (Guardian-verified)
    pub fn apply_consensus_optimization(
        &self,
        proposal: &ConsensusOptimizationProposal,
    ) -> Result<(), AxiomError> {
        if !proposal.guardian_pre_approved {
            return Err(AxiomError::AIProposalRejected {
                reason: "Proposal not pre-approved by Guardian".to_string(),
            });
        }

        if proposal.ai_confidence < 0.8 && proposal.requires_voting {
            return Err(AxiomError::AIProposalRejected {
                reason: "Proposal requires voting but confidence too low".to_string(),
            });
        }

        let mut consensus = self.consensus_ai.write();
        consensus.current_difficulty = proposal.proposed_difficulty;
        consensus.current_vdf_iterations = proposal.proposed_vdf;
        consensus.current_min_gas = proposal.proposed_min_gas;

        log::info!("🤖 Applied AI consensus optimization:");
        log::info!("   Difficulty: {} → {} ({:+.2}%)", proposal.current_difficulty, proposal.proposed_difficulty, proposal.difficulty_change_percent);
        log::info!("   VDF: {} → {} ({:+.2}%)", proposal.current_vdf, proposal.proposed_vdf, proposal.vdf_change_percent);
        log::info!("   Min Gas: {} → {} ({:+.2}%)", proposal.current_min_gas, proposal.proposed_min_gas, proposal.gas_change_percent);

        Ok(())
    }

    fn calculate_change_percent(old: u64, new: u64) -> f64 {
        if old == 0 {
            return 0.0;
        }
        ((new as f64 - old as f64) / old as f64) * 100.0
    }

    /// Activate emergency circuit breaker
    pub fn activate_circuit_breaker(&self, current_block: u64, reason: String) -> Result<(), AxiomError> {
        let mut breaker = self.emergency_circuit_breaker.write();

        if !breaker.is_active {
            breaker.is_active = true;
            breaker.activation_block = Some(current_block);
            breaker.reason = Some(reason.clone());
            breaker.auto_recovery_block = Some(current_block + 144);

            log::error!("🚨 EMERGENCY CIRCUIT BREAKER ACTIVATED at block {}", current_block);
            log::error!("   Reason: {}", reason);
            log::error!("   Auto-recovery: block {}", current_block + 144);
        }

        Ok(())
    }

    /// Deactivate circuit breaker (manual only)
    pub fn deactivate_circuit_breaker(&self) -> Result<(), AxiomError> {
        let mut breaker = self.emergency_circuit_breaker.write();

        if breaker.is_active {
            log::info!("✅ Emergency circuit breaker deactivated");
            breaker.is_active = false;
            breaker.activation_block = None;
            breaker.reason = None;
            breaker.auto_recovery_block = None;
        }

        Ok(())
    }

    /// Get Guardian statistics
    pub fn get_guardian_stats(&self) -> GuardianStats {
        let state = self.guardian_state.read();

        GuardianStats {
            ai_enabled: state.ai_enabled,
            auto_pilot_mode: state.auto_pilot_mode,
            total_ai_decisions: state.total_ai_decisions,
            guardian_vetoes: state.guardian_vetoes,
            veto_rate: if state.total_ai_decisions > 0 {
                (state.guardian_vetoes as f64 / state.total_ai_decisions as f64) * 100.0
            } else {
                0.0
            },
            last_veto_reason: state.last_veto_reason.clone(),
        }
    }
}

// ==================== GUARDIAN DECISION ====================

#[derive(Debug, Clone)]
pub struct GuardianDecision {
    pub approved: bool,
    pub veto_reason: Option<String>,
    pub action: GuardianAction,
    pub threat_assessment: ThreatAssessment,
}

#[derive(Debug, Clone)]
pub enum GuardianAction {
    Accept,
    AcceptMonitored,
    Quarantine { duration_blocks: u64 },
    Reject,
    AutoReject,
    RequireManualReview { threat_level: RiskLevel },
    ChainHalt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardianStats {
    pub ai_enabled: bool,
    pub auto_pilot_mode: bool,
    pub total_ai_decisions: u64,
    pub guardian_vetoes: u64,
    pub veto_rate: f64,
    pub last_veto_reason: Option<String>,
}

// ==================== CONSENSUS AI CONTROLLER ====================

impl ConsensusAIController {
    fn new() -> Self {
        Self {
            current_difficulty: 1000,
            current_vdf_iterations: 1_000_000,
            current_min_gas: 1000,
            difficulty_pid: PIDController::new(0.5, 0.1, 0.05, 0.95, 1.05),
            gas_pid: PIDController::new(0.3, 0.05, 0.02, 0.9, 1.1),
            vdf_pid: PIDController::new(0.2, 0.03, 0.01, 0.98, 1.02),
            block_time_history: Vec::with_capacity(1000),
            hashrate_history: Vec::with_capacity(1000),
            mempool_history: Vec::with_capacity(1000),
            optimization_history: Vec::new(),
        }
    }

    fn update_metrics(&mut self, blocks: &[BlockMetrics]) -> Result<(), AxiomError> {
        for block in blocks {
            self.block_time_history.push(block.block_time);
            self.hashrate_history.push(block.hashrate_estimate);

            if self.block_time_history.len() > 1000 {
                self.block_time_history.remove(0);
                self.hashrate_history.remove(0);
            }
        }
        Ok(())
    }

    fn calculate_difficulty_adjustment(&mut self) -> Result<u64, AxiomError> {
        let target_time = SovereignInvariants::TARGET_BLOCK_TIME_SECS as f64;
        let avg_time = self.block_time_history.iter().sum::<u64>() as f64
            / self.block_time_history.len() as f64;

        let error = (avg_time - target_time) / target_time;
        let pid_output = self.difficulty_pid.update(error, 1.0);

        let new_difficulty = (self.current_difficulty as f64 * pid_output) as u64;

        let max_change = (self.current_difficulty as f64 * 0.05) as u64;
        let bounded = if new_difficulty > self.current_difficulty {
            (self.current_difficulty + max_change).min(new_difficulty)
        } else {
            (self.current_difficulty.saturating_sub(max_change)).max(new_difficulty)
        };

        Ok(bounded.max(100))
    }

    fn calculate_vdf_adjustment(&mut self) -> Result<u64, AxiomError> {
        let avg_hashrate = if self.hashrate_history.is_empty() {
            1e12
        } else {
            self.hashrate_history.iter().sum::<f64>() / self.hashrate_history.len() as f64
        };

        let ratio = (avg_hashrate / 1e12).ln();
        let error = ratio * 0.1;
        let pid_output = self.vdf_pid.update(error, 1.0);

        let new_vdf = (self.current_vdf_iterations as f64 * pid_output) as u64;

        let max_change = (self.current_vdf_iterations as f64 * 0.02) as u64;
        let bounded = if new_vdf > self.current_vdf_iterations {
            (self.current_vdf_iterations + max_change).min(new_vdf)
        } else {
            (self.current_vdf_iterations.saturating_sub(max_change)).max(new_vdf)
        };

        Ok(bounded.max(SovereignInvariants::MINIMUM_VDF_ITERATIONS))
    }

    fn calculate_gas_adjustment(&mut self) -> Result<u64, AxiomError> {
        let avg_mempool = if self.mempool_history.is_empty() {
            500
        } else {
            self.mempool_history.iter().sum::<usize>() / self.mempool_history.len()
        };

        let error = (avg_mempool as f64 - 500.0) / 500.0;
        let pid_output = self.gas_pid.update(error, 1.0);

        let new_gas = (self.current_min_gas as f64 * pid_output) as u64;

        let max_change = (self.current_min_gas as f64 * 0.10) as u64;
        let bounded = if new_gas > self.current_min_gas {
            (self.current_min_gas + max_change).min(new_gas)
        } else {
            (self.current_min_gas.saturating_sub(max_change)).max(new_gas)
        };

        Ok(bounded.max(SovereignInvariants::MIN_TRANSACTION_FEE))
    }

    fn calculate_hashrate_trend(&self) -> Result<f64, AxiomError> {
        if self.hashrate_history.len() < 2 {
            return Ok(0.0);
        }

        let recent = *self.hashrate_history.last().unwrap();
        let older = self.hashrate_history[0];

        Ok((recent - older) / older)
    }

    fn calculate_mempool_congestion(&self) -> Result<f64, AxiomError> {
        if self.mempool_history.is_empty() {
            return Ok(0.0);
        }

        let avg = self.mempool_history.iter().sum::<usize>() as f64 / self.mempool_history.len() as f64;
        Ok((avg / 1000.0).min(1.0))
    }

    fn calculate_network_health_score(&self) -> Result<f64, AxiomError> {
        let block_time_score = self.calculate_block_time_stability()?;
        let hashrate_score = self.calculate_hashrate_stability()?;
        Ok((block_time_score + hashrate_score) / 2.0)
    }

    fn calculate_block_time_stability(&self) -> Result<f64, AxiomError> {
        if self.block_time_history.is_empty() {
            return Ok(0.5);
        }

        let target = SovereignInvariants::TARGET_BLOCK_TIME_SECS as f64;
        let avg = self.block_time_history.iter().sum::<u64>() as f64 / self.block_time_history.len() as f64;

        let deviation = ((avg - target) / target).abs();
        Ok((1.0 - deviation).max(0.0).min(1.0))
    }

    fn calculate_hashrate_stability(&self) -> Result<f64, AxiomError> {
        if self.hashrate_history.len() < 2 {
            return Ok(0.5);
        }

        let mean = self.hashrate_history.iter().sum::<f64>() / self.hashrate_history.len() as f64;
        let variance = self
            .hashrate_history
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>()
            / self.hashrate_history.len() as f64;

        let cv = variance.sqrt() / mean;
        Ok((1.0 - cv).max(0.0).min(1.0))
    }

    fn calculate_confidence(&self) -> Result<f64, AxiomError> {
        if self.block_time_history.len() < 144 {
            return Ok(0.5);
        }

        let data_quality = (self.block_time_history.len() as f64 / 1000.0).min(1.0);
        let stability = self.calculate_network_health_score()?;

        Ok((data_quality + stability) / 2.0)
    }

    fn calculate_expected_improvement(&self) -> Result<f64, AxiomError> {
        let target = SovereignInvariants::TARGET_BLOCK_TIME_SECS as f64;
        let current_avg =
            self.block_time_history.iter().sum::<u64>() as f64 / self.block_time_history.len() as f64;

        let current_deviation = ((current_avg - target) / target).abs();
        Ok((current_deviation * 50.0).min(20.0))
    }
}

// ==================== PID CONTROLLER ====================

impl PIDController {
    fn new(kp: f64, ki: f64, kd: f64, output_min: f64, output_max: f64) -> Self {
        Self {
            kp,
            ki,
            kd,
            integral: 0.0,
            previous_error: 0.0,
            output_min,
            output_max,
        }
    }

    fn update(&mut self, error: f64, dt: f64) -> f64 {
        self.integral += error * dt;
        let derivative = (error - self.previous_error) / dt;
        self.previous_error = error;

        let output = self.kp * error + self.ki * self.integral + self.kd * derivative;
        output.max(self.output_min).min(self.output_max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guardian_bridge_creation() {
        let engine = Arc::new(MultiLayerSecurityEngine::new(Default::default()));
        let bridge = AIGuardianBridge::new(engine);
        let stats = bridge.get_guardian_stats();
        assert_eq!(stats.total_ai_decisions, 0);
    }
}
