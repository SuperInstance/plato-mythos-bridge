/// Palaver consensus for room configuration — rooms negotiate config via iterative convergence.
/// Inspired by the African tradition of palaver: sitting together until agreement is reached.

use std::collections::HashMap;

/// A room's configuration proposal.
#[derive(Debug, Clone)]
pub struct RoomConfig {
    pub room_id: String,
    pub settings: HashMap<String, f64>,
}

impl RoomConfig {
    pub fn new(room_id: String) -> Self {
        Self {
            room_id,
            settings: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: f64) {
        self.settings.insert(key.to_string(), value);
    }

    pub fn get(&self, key: &str) -> Option<f64> {
        self.settings.get(key).map(|&v| v)
    }
}

/// Tracks convergence state during negotiation.
#[derive(Debug, Clone)]
pub struct ConvergenceState {
    pub round: u32,
    pub max_delta: f64,
    pub converged: bool,
}

/// Room consensus via palaver negotiation.
#[derive(Debug, Clone)]
pub struct RoomConsensus {
    proposals: Vec<RoomConfig>,
    convergence_threshold: f64,
    max_rounds: u32,
}

impl RoomConsensus {
    pub fn new(convergence_threshold: f64, max_rounds: u32) -> Self {
        Self {
            proposals: Vec::new(),
            convergence_threshold,
            max_rounds,
        }
    }

    /// Add a room with its config proposal.
    pub fn add_room(&mut self, _room_id: String, config: RoomConfig) {
        self.proposals.push(config);
    }

    /// Run negotiation for the given number of rounds.
    /// Each round, each setting converges toward the mean.
    /// Returns the consensus config and convergence state.
    pub fn negotiate(&mut self, rounds: u32) -> (RoomConfig, Vec<ConvergenceState>) {
        let mut history = Vec::new();

        for round in 1..=rounds.min(self.max_rounds) {
            if self.proposals.is_empty() {
                break;
            }

            // Collect all keys
            let mut all_keys: Vec<String> = Vec::new();
            for p in &self.proposals {
                for key in p.settings.keys() {
                    if !all_keys.contains(key) {
                        all_keys.push(key.clone());
                    }
                }
            }

            // Compute mean for each key
            let mut means: HashMap<String, f64> = HashMap::new();
            for key in &all_keys {
                let values: Vec<f64> = self
                    .proposals
                    .iter()
                    .filter_map(|p| p.settings.get(key))
                    .copied()
                    .collect();
                if !values.is_empty() {
                    means.insert(key.clone(), values.iter().sum::<f64>() / values.len() as f64);
                }
            }

            // Move each proposal toward the mean
            let mut max_delta = 0.0f64;
            for proposal in &mut self.proposals {
                for key in &all_keys {
                    if let (Some(current), Some(&mean)) =
                        (proposal.settings.get_mut(key), means.get(key))
                    {
                        let old = *current;
                        *current = *current + (mean - *current) * 0.5; // 50% convergence per round
                        let delta = (old - *current).abs();
                        if delta > max_delta {
                            max_delta = delta;
                        }
                    }
                }
            }

            let converged = max_delta < self.convergence_threshold;
            history.push(ConvergenceState {
                round,
                max_delta,
                converged,
            });

            if converged {
                break;
            }
        }

        // Final consensus: average of all proposals
        let consensus = self.compute_consensus();
        (consensus, history)
    }

    fn compute_consensus(&self) -> RoomConfig {
        let mut consensus = RoomConfig::new("consensus".to_string());

        let mut all_keys: Vec<String> = Vec::new();
        for p in &self.proposals {
            for key in p.settings.keys() {
                if !all_keys.contains(key) {
                    all_keys.push(key.clone());
                }
            }
        }

        for key in &all_keys {
            let values: Vec<f64> = self
                .proposals
                .iter()
                .filter_map(|p| p.settings.get(key))
                .copied()
                .collect();
            if !values.is_empty() {
                consensus.set(key, values.iter().sum::<f64>() / values.len() as f64);
            }
        }

        consensus
    }

    /// Get the number of participating rooms.
    pub fn room_count(&self) -> usize {
        self.proposals.len()
    }
}

/// Palaver consensus manager.
#[derive(Debug, Clone)]
pub struct PalaverConsensus;

impl PalaverConsensus {
    /// Create a new consensus negotiation with default settings.
    pub fn create_consensus(threshold: f64) -> RoomConsensus {
        RoomConsensus::new(threshold, 100)
    }
}
