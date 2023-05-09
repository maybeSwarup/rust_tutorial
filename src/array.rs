#![allow(unused)] // ignore unused variable warnings

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let arr_1 = [1, 2, 3, 4];
    println!("First: {}", arr_1[0]);
    println!("Length: {}", arr_1.len());
}
