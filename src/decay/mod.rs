//! # Vote Weight Decay Models
//!
//! Implements different models for how vote weights decrease over time,
//! encouraging early participation in the voting process.

use serde::{Deserialize, Serialize};

/// Models for how vote weights decay over time.
///
/// Each model provides a different curve for weight reduction:
/// - Linear: Steady decline from 1.0 to 0.1
/// - Exponential: Rapid early decline, slower later
/// - Stepped: Discrete weight levels based on voting phases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecayModel {
    /// Linear decay from 1.0 to 0.1 over the voting period
    Linear,
    /// Exponential decay with configurable rate parameter
    Exponential(f64),
    /// Stepped decay with discrete weight levels (1.0, 0.5, 0.1)
    Stepped,
}

/// Calculates the weight multiplier for a vote based on the decay model and timing.
///
/// All models enforce a minimum weight of 0.1 to ensure every vote has some influence.
///
/// # Arguments
/// * `model` - The decay model to use
/// * `t` - Time elapsed since voting started (seconds)
/// * `total` - Total voting period duration (seconds)
///
/// # Returns
/// Weight multiplier between 0.1 and 1.0
///
/// # Examples
/// ```
/// use verdyce_core::decay::{DecayModel, weight_calc};
///
/// // Linear decay at halfway point
/// let weight = weight_calc(&DecayModel::Linear, 1800, 3600);
/// assert!((weight - 0.5).abs() < 0.01);
///
/// // Exponential decay
/// let weight = weight_calc(&DecayModel::Exponential(0.001), 0, 3600);
/// assert!((weight - 1.0).abs() < 0.01);
/// ```
pub fn weight_calc(model: &DecayModel, t: u64, total: u64) -> f64 {
    match model {
        DecayModel::Linear => {
            let w = 1.0 - (t as f64 / total as f64);
            w.max(0.1)
        }
        DecayModel::Exponential(rate) => {
            let w = (-rate * t as f64).exp();
            w.max(0.1)
        }
        DecayModel::Stepped => {
            if t <= total / 3 {
                1.0
            } else if t <= (2 * total) / 3 {
                0.5
            } else {
                0.1
            }
        }
    }
}
