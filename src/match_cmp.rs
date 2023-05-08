#![allow(unused)] // ignore unused variable warnings

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let my_age = 21;
    let voting_age = 18;

    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("can't vote"),
        Ordering::Equal => println!("Your've gained right to vote!"),
        Ordering::Greater => println!("can vote"),
    }
}
