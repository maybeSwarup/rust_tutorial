#![allow(unused)] // ignore unused variable warnings

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn get_sum(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    println!("{}", get_sum(5, 4))
}
