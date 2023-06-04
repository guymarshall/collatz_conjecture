#![forbid(unsafe_code)]

use std::io;
use std::error::Error;
extern crate num;
use num::BigInt;

pub fn get_user_input(prompt: &str) -> Result<BigInt, Box<dyn Error>> {
    println!("{}", prompt);

    let mut user_input: String = String::new();

    io::stdin().read_line(&mut user_input)?;

    let number: BigInt = user_input.trim().parse().map_err(|_| "Please enter a valid integer!")?;

    if number <= BigInt::from(0) {
        return Err(From::from("The number must be greater than 0!"));
    }

    Ok(number)
}

pub fn input(prompt: &str) -> BigInt {
    loop {
        match get_user_input(prompt) {
            Ok(count) => {
                return count;
            },
            Err(error) => {
                println!("Error: {}", error);
            },
        };
    }
}