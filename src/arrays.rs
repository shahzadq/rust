#[allow(dead_code)]
fn basic_arrays() {
    let arr = [10, 20, 30, 40];
    let first = arr[0];
    println!("first array item: {}", first);

    for i in 0..arr.len() {
        println!("[{}] = {}", i, arr[i]);
    }

    println!("length {}", arr.len());
}

#[allow(dead_code)]
fn print_arrays() {
    let ints_arr = [1, 2, 3];
    let floats_arr = [1.1, 2.1, 3.1];
    let strs_arr = ["hello", "world"];
    let ints_ints_arr = [[1, 2], [3, 4]];

    // need special syntax for printing arrays
    println!("ints: {:?}", ints_arr);
    println!("floats: {:?}", floats_arr);
    println!("strings: {:?}", strs_arr);
    println!("ints ints: {:?}", ints_ints_arr);
}

#[allow(dead_code)]
fn main() {
    print_arrays();
}
