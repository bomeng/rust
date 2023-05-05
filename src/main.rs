#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    const SECOND_IN_MINUTE: u32 = 60;
    println!("{}", SECOND_IN_MINUTE);
    let x = 4;
    println!("x is: {}", x);
    {
        let x = 2;
        println!("x is: {}", x);
    }
    let x = x + 1;
    println!("x is: {}", x);

    println!("What is your name?");
    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin().read_line(&mut name)
        .expect("Didn't Receive Input");
    println!("Hello, {}! {}", name.trim_end(), greeting);

    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age: &str = "47";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
}
