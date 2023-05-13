#![allow(unused)] // ignore unused variable warnings

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn print_str(x: String) {
    println!("A string {}", x);
}

fn print_return_str(x: String) -> String {
    println!("A string {}", x);
    x
}

fn change_string(name: &mut String) {
    name.push_str(" is happy");
    println!("Message: {}", name);
}

fn main() {
    let str1 = String::from("World");
    let str2 = str1.clone();
    let str3 = print_return_str(str1);
    println!("str3 = {}", str3);
}
