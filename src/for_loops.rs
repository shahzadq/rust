#[allow(dead_code)]
fn basic() {
    // range is 0 to 4
    for i in 0..5 {
        println!("i: {}", i)
    }
}

#[allow(dead_code)]
fn even_or_odd() {
    // range is 1 to 5
    for i in 1..6 {
        println!(
            "{number} is {oddOrEven}",
            number = i,
            oddOrEven = if i % 2 == 0 { "even" } else { "odd" } // js equivalent: i % 2 === 0 ? "even" : "odd"
        )
    }
}

#[allow(dead_code)]
fn adding() {
    // let defines immutable variable (value can't rdirectly be changed)
    // use mut to make mutable
    let mut sum = 0;

    for i in 0..5 {
        sum += i;
    }

    println!("sum: {}", sum);
}

#[allow(dead_code)]
fn adding_float() {
    let mut sum = 0.0;

    for i in 0..5 {
        sum += i as f64;
    }

    println!("sum: {}", sum);
}

#[allow(dead_code)]
fn main() {
    adding_float();
}
