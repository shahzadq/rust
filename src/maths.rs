#[allow(dead_code)]
fn main() {
    // let pi: f64 = 3.1416;
    // let x = pi / 2.0;
    // let cosine = x.cos();
    // println!("cosine of {}: {}", x, cosine);

    let x = 2.0 * std::f64::consts::PI;
    let abs_diff = (x.cos() - 1.0).abs();
    assert!(abs_diff < 1e-10);
}
