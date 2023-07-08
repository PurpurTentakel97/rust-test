#![allow(unused)]

enum Address {
    Post { street: String, city: String },
    Email(String),
    Phone(u64),
    Handy(u32),
}

fn main() {
    let addresses = vec![
        Address::Email(String::from("Goldgasse@mail.de")),
        Address::Phone(0564245774),
        Address::Handy(56277487),
        Address::Post {
            street: "Superstr".to_string(),
            city: "Hanover".to_string(),
        },
    ];

    for address in &addresses {
        if let Address::Phone(_) = address {
            println!("Hör zu! ist ne Phone!!!!!!!1!!!!");
        }
        match &address {
            Address::Email(value) => println!("mail: {}", value),
            Address::Phone(value) => println!("phone: {}", value),
            Address::Handy(value) => println!("handy: {}", value),
            Address::Post { street, city } => println!("post: {}, {}", street, city),
        }
    }

    println!("Hello, world!");
    let a = 10;
    let b = 20;
    let c = mult(a, b);
    println!("{c}");
    println!("{}", mult(a, b));

    let a = 20;
    let _test = { 5 - 7 };

    let test2 = if a + b == 40 { 1 } else { 0 };
    println!("{test2}");

    let d = sub(a.try_into().unwrap_or(0), b.try_into().unwrap_or(0));

    let c = sub(
        match a.try_into() {
            Ok(value) => value,
            Err(_) => 0,
        },
        match b.try_into() {
            Ok(value) => value,
            Err(_) => 0,
        },
    );
    println!("{c}");

    let mut s = "super string".to_string();
    let size = length(&mut s);
    println!("{size}");
    println!("{s}");
}

fn mult(a: i64, b: i64) -> i64 {
    a * b
}
fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn length(s: &mut String) -> usize {
    add(s).len()
}

fn add(s: &mut String) -> &mut String {
    s.push_str("test");
    s
}
