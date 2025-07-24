use verdyce_core::decay::{DecayModel, weight_calc};

#[test]
fn test_linear_decay() {
    let decay = DecayModel::Linear;
    let w = weight_calc(&decay, 600, 1800);
    let expected = 1.0 - (600.0 / 1800.0);
    let epsilon = 0.001;
    assert!((w - expected).abs() < epsilon);
}

#[test]
fn test_exponential_decay() {
    let decay = DecayModel::Exponential(0.001);
    let w = weight_calc(&decay, 300, 1800);
    let expected = (-0.001f64 * 300.0).exp();
    let epsilon = 0.001;
    assert!((w - expected).abs() < epsilon);
}

#[test]
fn test_stepped_decay() {
    let decay = DecayModel::Stepped;
    let w1 = weight_calc(&decay, 200, 1800);
    let w2 = weight_calc(&decay, 800, 1800);
    let w3 = weight_calc(&decay, 1400, 1800);
    assert_eq!(w1, 1.0);
    assert_eq!(w2, 0.5);
    assert_eq!(w3, 0.1);
}
