#[derive(Debug, Clone, PartialEq)]
pub enum DecayModel {
    Linear,
    Exponential(f64),
    Stepped
}

pub fn weight_calc(model: &DecayModel, t: u64,total: u64) -> f64 {
    match model {
        DecayModel::Linear => {
            let w = 1.0 - (t as f64 / total as f64);
            w.max(0.1)
        }
        DecayModel::Exponential(rate) => {
            let w= (-rate * t as f64).exp();  
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