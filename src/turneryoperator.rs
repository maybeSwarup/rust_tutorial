#![allow(unused)] // ignore unused variable warnings

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let my_age = 27;
    let can_vote = if my_age >= 18 { true } else { false };

    println!("Can Vote {}!", can_vote);
}
