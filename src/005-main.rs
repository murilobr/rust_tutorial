#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};


fn main() {
    let num_1: f32 = 1.111111111111111;
    println!("f32: {}", num_1 + 0.11111111111111);

    let num_2: f64 = 1.111111111111111;
    println!("f64: {}", num_2 + 0.11111111111111);

    let num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 % 4 = {}", num_3 % num_4);

    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random: {}", random_num);
}