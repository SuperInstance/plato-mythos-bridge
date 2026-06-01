/// Kintsugi approach to system health — errors are golden seams.
/// System health IS a repair problem, not a monitoring problem.

use std::time::{SystemTime, UNIX_EPOCH};

/// A golden seam representing a repaired error in the system.
#[derive(Debug, Clone)]
pub struct GoldenSeam {
    /// How severe the original fault was (0.0 = trivial, 1.0 = critical)
    pub severity: f64,
    /// Where in the system hierarchy the fault occurred
    pub position: String,
    /// How well the repair was executed (0.0 = poor, 1.0 = perfect)
    pub repair_quality: f64,
    /// Timestamp of the repair
    pub repaired_at: u64,
    /// Description of the repair
    pub description: String,
}

impl GoldenSeam {
    /// The beauty score: a well-repaired severe fault is more beautiful than an untested system.
    pub fn beauty(&self) -> f64 {
        let weight = 0.4 * self.severity + 0.6 * self.repair_quality;
        weight.clamp(0.0, 1.0)
    }
}

/// A health report entry from PLATO monitoring.
#[derive(Debug, Clone)]
pub struct HealthReport {
    pub component: String,
    pub status: HealthStatus,
    pub message: String,
    pub metric_value: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Faulted,
    Unknown,
}

impl HealthStatus {
    pub fn severity(&self) -> f64 {
        match self {
            HealthStatus::Healthy => 0.0,
            HealthStatus::Degraded => 0.4,
            HealthStatus::Faulted => 0.9,
            HealthStatus::Unknown => 0.6,
        }
    }
}

/// Kintsugi-style health analyzer.
#[derive(Debug, Clone)]
pub struct KintsugiHealth {
    seams: Vec<GoldenSeam>,
    repair_history_count: usize,
}

impl KintsugiHealth {
    pub fn new() -> Self {
        Self {
            seams: Vec::new(),
            repair_history_count: 0,
        }
    }

    /// Treat a health report as a repair problem: every issue is an opportunity for a golden seam.
    pub fn health_as_repair(&mut self, health_report: &HealthReport) -> GoldenSeam {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let severity = health_report.status.severity();
        let position = health_report.component.clone();

        // Repair quality improves with experience
        let experience_bonus = (self.repair_history_count as f64 * 0.02).min(0.3);
        let base_quality = match health_report.status {
            HealthStatus::Healthy => 1.0,
            HealthStatus::Degraded => 0.6 + experience_bonus,
            HealthStatus::Faulted => 0.3 + experience_bonus,
            HealthStatus::Unknown => 0.5 + experience_bonus,
        };

        let seam = GoldenSeam {
            severity,
            position,
            repair_quality: base_quality.min(1.0),
            repaired_at: now,
            description: health_report.message.clone(),
        };

        self.seams.push(seam.clone());
        self.repair_history_count += 1;
        seam
    }

    /// Process multiple health reports at once.
    pub fn repair_all(&mut self, reports: &[HealthReport]) -> Vec<GoldenSeam> {
        reports.iter().map(|r| self.health_as_repair(r)).collect()
    }

    /// Overall system beauty: a system with well-repaired faults is more beautiful
    /// than a system that was never tested.
    pub fn system_beauty(&self) -> f64 {
        if self.seams.is_empty() {
            return 0.5; // Untested — not beautiful, not ugly
        }
        let avg: f64 = self.seams.iter().map(|s| s.beauty()).sum::<f64>() / self.seams.len() as f64;
        // Bonus for having repaired many things
        let volume_bonus = (self.seams.len() as f64 * 0.01).min(0.2);
        (avg + volume_bonus).min(1.0)
    }

    /// Get all golden seams.
    pub fn seams(&self) -> &[GoldenSeam] {
        &self.seams
    }

    /// Count seams by severity range.
    pub fn severity_distribution(&self) -> (usize, usize, usize) {
        let low = self.seams.iter().filter(|s| s.severity < 0.3).count();
        let mid = self.seams.iter().filter(|s| s.severity >= 0.3 && s.severity < 0.7).count();
        let high = self.seams.iter().filter(|s| s.severity >= 0.7).count();
        (low, mid, high)
    }
}

impl Default for KintsugiHealth {
    fn default() -> Self {
        Self::new()
    }
}
