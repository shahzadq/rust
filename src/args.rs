#[allow(dead_code)]
fn dump_all() {
    // get all args passed when running scrips
    // example: ./args 42 'hello world' frodo -> ['./args', '42', 'hello world', 'frodo']
    for arg in std::env::args() {
        println!("'{}'", arg);
    }
}

#[allow(dead_code)]
fn to_vec() {
    // convert all args to a vector - skips first item (the file path)
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() > 0 {
        println!("We have args!")
    }
}

#[allow(dead_code)]
fn main() {
    to_vec();
}
