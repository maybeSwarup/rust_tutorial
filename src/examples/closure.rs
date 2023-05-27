#![allow(unused)] // ignore unused variable warnings

use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    // let var_name = |parameters| -> return_type{BODY}
    let can_vote = |age: i32| age >= 18;
    println!("Can vote; {}", can_vote(8));
}
