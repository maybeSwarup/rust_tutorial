#![allow(unused)] // ignore unused variable warnings

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let num_1: f32 = 1.111111111111111;
    println!("f32: {}", num_1 + 0.111111111111111); //  1.2222223
    let num_2: f64 = 1.111111111111111;
    println!("f64: {}", num_2 + 0.111111111111111); // 1.2222222222222219
}
