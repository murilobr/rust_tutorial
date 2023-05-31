#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};


fn main() {
    let mut str1 = String::new();
    str1.push('A');
    str1.push_str(" word");
    for word in str1.split_whitespace() {
        println!("{}", word);
    }

    let str2 = str1.replace("A", "Another");
    println!("{}", str2);

    let str3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = str3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }

    let str4: &str = "Random string";
    let mut str5: String = str4.to_string();
    println!("{}", str5);

    let byte_arr1 = str5.as_bytes();
    let str6 = &str5[0..6];
    println!("String length: {}", str6.len());
    str5.clear();

    let str7 = String::from("Just some");
    let str8 = String::from(" words");
    let str9 = str7 + &str8;
    for char in str9.bytes() {
        println!("{}", char);
    }
}