#![allow(unused)] // ignore unused variable warnings

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let age = 19;

    if (age >= 1) && (age <= 18) {
        println!("Important Birthday")
    } else if (age == 21) || (age == 50) {
        println!("Not Much Important Birthday")
    } else if age > 65 {
        println!("You are definitely not a child anymore!")
    } else {
        println!("You are possibly/presumed dead person.")
    }
}
