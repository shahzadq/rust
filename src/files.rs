use std::{
    env,
    fs::File,
    io::{self, Read},
};

#[allow(dead_code)]
fn good_or_bad(good: bool) -> Result<i32, String> {
    if good {
        Ok(42)
    } else {
        Err("bad".to_string())
    }
}

#[allow(dead_code)]
fn test_good_or_bad() {
    println!("true: {:?}", good_or_bad(true)); // -> Ok(42)
    println!("false: {:?}", good_or_bad(false)); // -> Err("bad")

    match good_or_bad(true) {
        Ok(n) => println!("Cool, I got {}", n),
        Err(e) => println!("Huh, I just got {}", e),
    }
}

#[allow(dead_code)]
fn read_file_from_arg() {
    // get a filename as an arg
    let first = env::args().nth(1).expect("please supply a filename");

    let mut file = File::open(&first).expect("can't open file");

    let mut text = String::new();
    file.read_to_string(&mut text).expect("can't read file");

    println!("file has {} bytes", text.len());
}

#[allow(dead_code)]
fn read_to_string(filename: &str) -> Result<String, io::Error> {
    let mut file = match File::open(&filename) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Ok(_) => Ok(text),
        Err(e) => Err(e),
    }
}

#[allow(dead_code)]
fn neater_read_to_string(filename: &str) -> io::Result<String> {
    let mut file = File::open(&filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

#[allow(dead_code)]
fn safe_read_file_from_arg() {
    let file = env::args().nth(1).expect("please supply a filename");
    let text = read_to_string(&file).expect("bad file name");
    println!("file has {} bytes", text.len());
}

#[allow(dead_code)]
fn main() {
    safe_read_file_from_arg();
}
