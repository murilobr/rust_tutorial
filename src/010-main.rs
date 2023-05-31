#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};


fn main() {
    let int_u8: u8 = 5;
    let in2_u8: u8 = 4;
    let int3_u32: u32 = (int_u8 as u32) + (in2_u8 as u32);
}