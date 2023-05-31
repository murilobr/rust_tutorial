#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fmt::write;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

// Stack: Stores values in a last in first out format
// Data on the stack must have a defined fixed size

fn main() {
    // let b_int1 = Box::new(10);
    // println!("b_int1 = {}", b_int1);

    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }
    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode { left: None, right: None, key, }
        }
        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }
        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }

    let node1 = TreeNode::new(1)
        .left(TreeNode::new(2))
        .right(TreeNode::new(3));
}