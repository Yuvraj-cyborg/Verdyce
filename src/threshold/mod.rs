//! # Approval Threshold Models
//!
//! Implements different models for how approval thresholds change over time,
//! allowing for quick early consensus while requiring higher scrutiny later.

use serde::{Deserialize, Serialize};

/// Models for how approval thresholds change over time.
///
/// Each model provides a different progression curve:
/// - Linear: Steady increase over time
/// - Exponential: Rapid early increase, slower later
/// - Sigmoid: S-curve progression with smooth transitions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ThresholdModel {
    /// Linear threshold progression: `threshold = t * rate + start`
    ///
    /// # Parameters
    /// * `rate` - How quickly threshold increases per unit time
    /// * `start` - Starting threshold value
    Linear(f64, f64),

    /// Exponential threshold progression with asymptotic approach
    ///
    /// # Parameters  
    /// * `rate` - Growth rate parameter
    /// * `base` - Base threshold value
    Exponential(f64, f64),

    /// Sigmoid (S-curve) threshold progression
    ///
    /// # Parameters
    /// * `rate` - Steepness of the curve
    /// * `floor` - Minimum threshold value
    Sigmoid(f64, f64),
}

/// Calculates the approval threshold at a given time using the specified model.
///
/// All models enforce bounds between 0.35 and 0.9 to ensure reasonable
/// threshold ranges regardless of parameters.
///
/// # Arguments
/// * `model` - The threshold model to use
/// * `t` - Time elapsed since voting started (seconds)
/// * `total` - Total voting period duration (seconds)
///
/// # Returns
/// Approval threshold between 0.35 and 0.9
///
/// # Examples
/// ```
/// use verdyce_core::threshold::{ThresholdModel, threshold_calc};
///
/// // Linear threshold starting at 0.5, increasing by 0.0001 per second
/// let threshold = threshold_calc(&ThresholdModel::Linear(0.0001, 0.5), 1800, 3600);
/// assert!((threshold - 0.68).abs() < 0.01);
/// ```
pub fn threshold_calc(model: &ThresholdModel, t: u64, total: u64) -> f64 {
    match model {
        ThresholdModel::Linear(r, s) => {
            let thres = t as f64 * r + s;
            thres.clamp(0.35, 0.9)
        }
        ThresholdModel::Exponential(r, s) => {
            let growth = 1.0 - (-r * t as f64).exp();
            let thres = s + (1.0 - s) * growth;
            thres.clamp(0.35, 0.9)
        }
        ThresholdModel::Sigmoid(r, s) => {
            let x = t as f64 / total as f64;
            let sigmoid = 1.0 / (1.0 + (-r * (x - 0.5)).exp());
            let thres = s + (1.0 - s) * sigmoid;
            thres.clamp(0.35, 0.9)
        }
    }
}
