#![allow(unused)] // ignore unused variable warnings

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn get_2(x: i32) -> (i32, i32) {
    return (x + 1, x + 2);
}

fn main() {
    let (val_1, val_2) = get_2(3);
    println!("Nums: {} {}", val_1, val_2);
}
