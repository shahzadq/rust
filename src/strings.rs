fn dump(str: &str) {
    println!("str: '{}'", str);
}

#[allow(dead_code)]
fn basic() {
    let text = "hello world";
    let str = text.to_string();

    dump(text);
    dump(&str);
}

#[allow(dead_code)]
fn push_and_pop() {
    let mut s = String::new();

    s.push('H');
    s.push_str("ello");
    s.push(' ');
    s += "World!";

    s.pop();

    assert_eq!(s, "Hello World");

    println!("{}", s);
}

fn array_to_str(arr: &[i32]) -> String {
    let mut res = '['.to_string();
    for v in arr {
        res += &v.to_string();
        res.push(',');
    }
    res.pop();
    res.push(']');
    res
}

#[allow(dead_code)]
fn array_to_str_usage() {
    let arr = array_to_str(&[10, 20, 30]);
    let res = format!("hello {}", arr);

    assert_eq!(res, "hello [10,20,30]");
    println!("array as string: {}", arr);
}

#[allow(dead_code)]
fn slices_notation() {
    let text = "static";
    let string = "dynamic".to_string();

    let text_s = &text[1..];
    let string_s = &string[2..4];

    println!("slices {:?} {:?}", text_s, string_s);
}

#[allow(dead_code)]
fn multilingual() {
    let multilingual = "Hi! ¡Hola! привет!";
    for ch in multilingual.chars() {
        print!("'{}'", ch);
    }
    println!("");
    println!("len: {}", multilingual.len());
    println!("count: {}", multilingual.chars().count());

    let maybe = multilingual.find('п');
    if maybe.is_some() {
        let hi = &multilingual[maybe.unwrap()..];
        println!("Russian hi: {}", hi);
    }
}

#[allow(dead_code)]
fn multilingual_match() {
    let multilingual = "Hi! ¡Hola! привет!";

    match multilingual.find('п') {
        Some(idx) => {
            let hi = &multilingual[idx..];
            println!("Russian hi: {}", hi)
        }
        None => println!("Couldnt find russian greeting..."),
    }

    // if not interested in faliure
    if let Some(idx) = multilingual.find('п') {
        println!("Russian hi: {}", &multilingual[idx..]);
    }

    // C like switch statement
    // let text = match n {
    //     0 => "zero",
    //     1 => "one",
    //     2 => "two",
    //     _ => "many",
    // };
    // or as a range
    // let text = match n {
    //     0..3 => "small",
    //     4..6 => "medium",
    //     _ => "large"
    // };
}

#[allow(dead_code)]
fn whitespace_and_collect() {
    let text = "the red fox and the lazy dog";
    let words: Vec<&str> = text.split_whitespace().collect();

    println!("text: {} is made up of words: {:?}", text, words);

    let mut words = Vec::new();
    words.extend(text.split_whitespace());

    let stripped: String = text.chars().filter(|ch| !ch.is_whitespace()).collect();

    println!("{:?}", stripped);
}

#[allow(dead_code)]
fn main() {
    multilingual_match();
}
