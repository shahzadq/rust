use std::fmt::format;

#[allow(dead_code)]
fn sum(values: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0..values.len() {
        res += values[i];
    }
    res
}

#[allow(dead_code)]
fn slice_arr() {
    let ints_arr = [1, 2, 3, 4, 5];
    let slice1 = &ints_arr[0..2];
    let slice2 = &ints_arr[1..]; // open range - start at 1 to end

    println!("ints: {:?}", ints_arr);
    println!("slice 1: {:?}", slice1);
    println!("slice 2: {:?}", slice2);
}

#[allow(dead_code)]
fn maybe_slice_get(slice: &[i32], idx: usize) -> i32 {
    // longwinded solution
    // let maybe = slice.get(idx);
    // let item = if maybe.is_some() { *maybe.unwrap() } else { -1 };
    // item

    *slice.get(idx).unwrap_or(&-1)
}

fn print_slice_get(pre: &str, item: Option<&i32>) {
    let str = format(format_args!(
        "{}: {:?}, is some: {}, is none: {}",
        pre,
        item,
        item.is_some(),
        item.is_none()
    ));

    if item.is_some() {
        println!("{}, unwrapped: {}", str, item.unwrap());
    } else {
        println!("{}", str)
    }
}

#[allow(dead_code)]
fn slice_get() {
    let ints_arr = [1, 2, 3, 4, 5];
    let slice = &ints_arr;
    let first_item = slice.get(0);
    let last_item = slice.get(5);

    print_slice_get("first item", first_item);
    print_slice_get("last item", last_item);
}

#[allow(dead_code)]
fn main() {
    // implementation of sum fn
    // let arr = [10, 20, 30, 40];
    // let res = sum(&arr);
    // println!("sum {}", res);

    slice_get();
}
