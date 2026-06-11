use string::String;

#[test]
fn new_empty() {
    let str = String::new();

    assert_eq!(str.len(), 0);
    assert_eq!(str.capacity(), 0);
}

#[test]
fn new_with_capacity() {
    let str = String::new_with_capacity(16);

    assert_eq!(str.len(), 0);
    assert_eq!(str.capacity(), 16);
}

#[test]
fn hello_world() {
    let mut str = String::new();
    str.push_str("Hello, World!");

    assert_eq!(str.as_bytes(), "Hello, World!".as_bytes());
}

#[test]
fn hello_world_mixed() {
    let mut str = String::new();

    str.push_str("Hello");
    str.push_char(',');
    str.push_char(' ');
    str.push_str("World");
    str.push_char('!');

    assert_eq!(str.as_bytes(), "Hello, World!".as_bytes());
}

#[test]
fn new_string_with_million_bytes() {
    let mut str = String::new();

    for _ in 0..1_000_000 {
        str.push_char(' ');
    }

    assert_eq!(str.len(), 1_000_000);
    assert_eq!(str.capacity(), 2usize.pow(20));
}

#[test]
fn new_with_cap_million_bytes() {
    let capacity: usize = 1_000_000;
    let mut str = String::new_with_capacity(capacity);

    for _ in 0..1_000_000 {
        str.push_char(' ');
    }

    assert_eq!(str.len(), 1_000_000);
    assert_eq!(str.capacity(), capacity);
}

#[test]
fn get_len() {
    let mut str = String::new();
    str.push_str("Hello");

    assert_eq!(str.len(), 5);
}

#[test]
fn get_len2() {
    let mut str = String::new();

    str.push_char('H');
    str.push_char('e');
    str.push_char('l');
    str.push_char('l');
    str.push_char('o');

    assert_eq!(str.len(), 5);
}

#[test]
fn get_capacity() {
    let mut str = String::new();
    str.push_str("Hello");

    assert_eq!(str.capacity(), 8);
}

#[test]
fn get_capacity2() {
    let mut str = String::new();

    str.push_char('H');
    str.push_char('e');
    str.push_char('l');
    str.push_char('l');
    str.push_char('o');

    assert_eq!(str.capacity(), 8);
}

#[test]
fn clear_string() {
    let mut str = String::new();
    str.push_str("Hello");

    str.clear();

    assert_eq!(str.len(), 0);
    assert_eq!(str.capacity(), 8);
}

#[test]
fn erase_string() {
    let mut str = String::new();
    str.push_str("Hello");

    str.erase();

    assert_eq!(str.len(), 0);
    assert_eq!(str.capacity(), 0);
}

#[test]
fn string_as_bytes() {
    let bytes: Vec<u8> = vec![72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33];
    let mut str = String::new();
    str.push_str("Hello, World!");

    assert_eq!(str.as_bytes(), &bytes);
}

#[test]
fn string_as_str() {
    let comp_str = "Hello, World!";
    let mut str = String::new();
    str.push_str("Hello, World!");

    assert_eq!(str.as_str(), comp_str);
}

#[test]
fn from_str_ref() {
    let str_ref = "Hello";
    let str = String::from(str_ref);

    assert_eq!(str.as_str(), str_ref);
    assert_eq!(str.len(), 5);
    assert_eq!(str.capacity(), 5);
}

#[test]
fn clone() {
    let str1 = String::from("This is a test str");

    let str2 = str1.clone();

    assert_eq!(str1.as_str(), str2.as_str());
    assert_eq!(str1.as_bytes(), str2.as_bytes());
    assert_eq!(str1.len(), str2.len());
}

#[test]
fn eq() {
    let str1 = String::from("This is a test String");
    let str2 = String::from("This is a test String");
    let str3 = String::from("This is a different test String");

    assert!(str1 == str2);
    assert!(str1 != str3);
    assert!(str1 == "This is a test String");
}

#[test]
fn shrink_to_fit() {
    let mut str = String::new_with_capacity(16);
    str.push_str("Hello, World!");

    assert_eq!(str.len(), 13);
    assert_eq!(str.capacity(), 16);

    str.shrink_to_fit();

    assert_eq!(str.len(), 13);
    assert_eq!(str.capacity(), 13);
}
