#![allow(unused)] // ignore unused variable warnings

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let age2 = 165;

    match age2 {
        1..=18 => println!("Imortant Birthday!"),
        21 | 51 => println!("Not important birthday"),
        65..=i32::MAX => println!("Not at all important birthday!"),
        _ => println!("What is Birthday?"),
    }
}
