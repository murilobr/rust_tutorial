#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    #[derive(Debug)]
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }

    let mut bob = Customer {
        name: String::from("Bob Smith"),
        address: String::from("555 Main St"),
        balance: 234.50,
    };
    bob.address = String::from("505 Main St");
    
    println!("{:?}", bob);

    // struct Rectangle<T, U> {
    //     length: T,
    //     height: U,
    // }
    // let rec = Rectangle {
    //     length: 4,
    //     height: 10.5,
    // };

    trait Shape {
        fn new(lenght: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }
    struct Rectangle { length: f32, width: f32 };
    struct Circle { length: f32, width: f32 };

    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle { length, width };
        }

        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }

    const PI: f32 = 3.141592;
    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle { length, width };
        }

        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }

    let rec: Rectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);

    println!("Rec area: {}", rec.area());
    println!("Circ area: {}", circ.area());
}