#![allow(unused)] // ignore unused variable warnings

use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let mut arr_it = [1, 2, 3, 4];
    for val in arr_it.iter() {
        println!("{}", val);
    }

    let mut iter1 = arr_it.iter();
    println!("1st; {:?}", iter1.next());
    println!("2nd; {:?}", iter1.next());
    println!("3rd; {:?}", iter1.next());
}
