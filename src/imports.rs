use std::f64::consts;

#[allow(dead_code)]
fn main() {
    let x = 2.0 * consts::PI;
    let abs_diff = (x.cos() - 1.0).abs();
    assert!(abs_diff < 1e-10);
}
