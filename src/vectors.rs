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
fn initialise_vector_simple() {
    let mut v1 = vec![10, 20, 30, 40];
    v1.pop();

    let mut v2 = Vec::new();
    v2.push(10);
    v2.push(20);
    v2.push(30);

    assert_eq!(v1, v2);

    v2.extend(0..2);
    assert_eq!(v2, &[10, 20, 30, 0, 1]);
}

#[allow(dead_code)]
fn sort_and_deduplicate() {
    let mut v = vec![1, 10, 5, 1, 2, 11, 2, 40];
    println!("initial v: {:?}", v);
    v.sort();
    v.dedup();
    println!("v sorted and deduped: {:?}", v);
}

#[allow(dead_code)]
fn main() {
    sort_and_deduplicate();
}
