/*
Copyright (c) 2024 fbrega10
MIT License
* StringUtils
*/
use string_utils::string_utils::{eliminate_whitespaces, left, right, substring, trim};

#[test]
pub fn left_test() {
    assert_eq!(
        String::from("Hel"),
        left(&mut String::from("Hello world!"), 3).expect("cannot parse string")
    )
}

#[test]
pub fn right_test() {
    assert_eq!(
        String::from("ld!"),
        right(&mut String::from("Hello world!"), 3).expect("cannot parse string")
    )
}

#[test]
pub fn trim_test() {
    let mut s: String = String::from("     Hello world!    ");
    trim(&mut s);
    assert_ne!(String::from("hello world").len(), s.len())
}

#[test]
pub fn substring_test() {
    let s = String::from("hello world");
    assert_eq!(
        String::from("hello"),
        substring(&s, 0, 4).expect("error in the substring")
    )
}

#[test]
pub fn eliminate_whitespace_test() {
    let mut s = String::from("hello world");
    eliminate_whitespaces(&mut s);
    assert_eq!(String::from("helloworld").len(), s.len())
}
