#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fmt::write;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    // clojure
    // let var_name = |parameters| -> return_type

    // let can_vote = |age: i32| {
    //     age >= 18
    // };
    // println!("Can vote: {}", can_vote(8));

    // let mut samp1 = 5;
    // let print_var = || println!("samp1 = {}", samp1);
    // print_var();
    // samp1 = 10;
    // let mut change_var = || samp1 += 1;
    // change_var();
    // println!("samp1 = {}", samp1);
    // samp1 = 10;
    // println!("samp1 = {}", samp1)

    fn use_func<T>(a: i32, b: i32, func: T) -> i32 where T: Fn(i32, i32) -> i32 {
        func(a, b)
    }
    let sum = |a, b| a+b;
    let prod = |a, b| a*b;
    println!("5 + 4 = {}", use_func(5, 4, sum));
    println!("5 * 4 = {}", use_func(5, 4, prod));
}