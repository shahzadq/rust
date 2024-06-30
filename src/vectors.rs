#[allow(dead_code)]
fn basic_vector() {
    let mut v = Vec::new();

    v.push(10);
    v.push(20);
    v.push(30);

    let first = v[0];
    let maybe_first = v.get(0);

    println!("v: {:?}", v);
    println!("first: {}", first);
    println!("maybe first: {:?}", maybe_first);
}

fn dump(arr: &[i32]) {
    println!("arr is {:?}", arr);
}

#[allow(dead_code)]
fn vectors_and_slices() {
    let mut v = Vec::new();

    v.push(10);
    v.push(20);
    v.push(30);

    dump(&v);

    let slice = &v[1..];
    println!("slice is {:?}", slice);
}

#[allow(dead_code)]
fn main() {
    vectors_and_slices();
}
