#![forbid(unsafe_code)]

extern crate num;
use num::BigInt;

use std::io;

pub fn get_user_input(prompt: &str) -> BigInt {
    println!("{}", prompt);

    let mut user_input: String = String::new();

    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    user_input.trim().parse().expect("Please enter an integer!")
}