#[allow(dead_code)]
fn sqr(x: f64) -> f64 {
    return x * x;
}

#[allow(dead_code)]
fn sqr_simplified(x: f64) -> f64 {
    // don't need explicit return - don't add semicolons
    x * x
}

// example of recursive function
#[allow(dead_code)]
fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

#[allow(dead_code)]
fn by_ref(x: &i32) -> i32 {
    *x + 1
}

#[allow(dead_code)]
fn modify_arg(x: &mut f64) {
    *x = 1.0;
}

fn main() {
    // let arg = 2.0;
    // let res = sqr_simplified(arg);
    // println!("sqr {}: {}", arg, res);

    // let i = 10;
    // let res1 = by_ref(&i);
    // let res2 = by_ref(&41);
    // println!("{} {}", res1, res2);

    let mut res = 0.0;
    modify_arg(&mut res);
    println!("res is {}", res);
}
