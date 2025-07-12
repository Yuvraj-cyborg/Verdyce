use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ThresholdModel {
    Linear(f64,f64),
    Exponential(f64,f64),
    Sigmoid(f64,f64)
}
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


