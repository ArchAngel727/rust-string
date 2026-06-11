mod string;

use crate::string::String;

fn main() {
    let mut a = String::new();

    a.push_str("Hello, world");
    a.push('!');

    println!("{}", a);
    println!("a has len {}", a.len());
}
