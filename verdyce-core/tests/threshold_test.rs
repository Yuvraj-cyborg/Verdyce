use verdyce_core::threshold::{ThresholdModel, threshold_calc};

#[test]
fn test_linear_threshold() {
    let model = ThresholdModel::Linear(0.5, 0.1);
    let t = 10;
    let total = 100;
    let thres = threshold_calc(&model, t, total);
    let expected = (0.5 * t as f64 + 0.1).min(0.9).max(0.35);
    assert!((thres - expected).abs() < 0.001)
}

#[test]
fn test_exponential_threshold() {
    let model = ThresholdModel::Exponential(0.5, 0.1);
    let t = 10;
    let total = 100;
    let thres = threshold_calc(&model, t, total);
    let expected = (0.1 + (1.0 - (-0.5 * t as f64).exp())).min(0.9).max(0.35);
    assert!((thres - expected).abs() < 0.001)
}

#[test]
fn test_sigmoid_threshold() {
    let model = ThresholdModel::Sigmoid(0.5, 0.1);
    let t = 10;
    let total = 100;
    let thres = threshold_calc(&model, t, total);
    let x = t as f64 / total as f64;
    let sigmoid = 1.0 / (1.0 + (-0.5 * (x - 0.5)).exp());
    let expected = 0.1 + (1.0 - 0.1) * sigmoid;
    assert!((thres - expected).abs() < 0.001)
}