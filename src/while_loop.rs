#![allow(unused)] // ignore unused variable warnings

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_idx = 0;

    while loop_idx < arr_2.len() {
        println!("Arr: {}", arr_2[loop_idx]);
        loop_idx += 1;
    }
}
