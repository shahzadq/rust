#[allow(dead_code)]
fn basic_iter() {
    let mut iter = 0..3;

    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
}

#[allow(dead_code)]
fn arr_iter() {
    let arr = [10, 20, 30];
    for i in arr.iter() {
        println!("{}", i);
    }

    let slice = &arr;
    for i in slice {
        println!("{}", i);
    }
}

#[allow(dead_code)]
fn sums() {
    let sum: i32 = (0..5).sum();
    println!("sum: {}", sum);

    let sum: i64 = [10, 20, 30].iter().sum();
    println!("sum: {}", sum);
}

#[allow(dead_code)]
fn slice_windows() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;

    for s in slice.windows(2) {
        println!("window: {:?}", s);
    }
}

#[allow(dead_code)]
fn main() {
    slice_windows();
}
