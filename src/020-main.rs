#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fmt::write;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

// Result has 2 varients Ok and Err
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
// Where T represents the data typeof the value returns
// and E the type of error

fn main() {
    // panic!("Terrible Error");

    // let lil_arr = [1,2];
    // println!("{}", lil_arr[10]);

    let path = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file: {:?}", error);
        }
    };
    write!(output, "Just some\nRandom words").expect("Failed to write to file");
    
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    let output2 = File::create("rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", e),
            },
            _other_error => panic!("Problem opening file: {:?}", error),
        }
    };
}