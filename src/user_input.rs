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

/*
extern crate num;
use num::BigInt;

fn number_to_vector(number: i32) -> Vec<BigInt> {
    let mut numbers: Vec<BigInt> = Vec::new();

    (1..=number).into_iter().for_each(|i| {
        numbers.push(BigInt::from(i));
    });

    numbers
}

pub fn factorial(number: i32) -> BigInt {
    let numbers: Vec<BigInt> = number_to_vector(number);

    let factorial: BigInt = numbers.iter().fold(BigInt::from(1), |acc, x| acc * x);

    factorial
}
*/