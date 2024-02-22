#![forbid(unsafe_code)]

mod collatz;
mod file_utilities;

extern crate num;

use std::ops::Add;
use std::str::FromStr;
use num::{BigInt, Integer};
use crate::collatz::collatz;
use crate::file_utilities::{read_last_values, write_to_file};
use thousands::Separable;

fn main() {
    let mut number: BigInt = BigInt::from_str("1").unwrap();
    let mut highest_steps: BigInt = BigInt::from_str("0").unwrap();

    if let Ok(res) = read_last_values("collatz_data.csv") {
        number = res[0].clone();
        highest_steps = res[1].clone();
    }
    
    loop {
        let steps: BigInt = collatz(&number);
        
        if number.is_even() {
            number = number.add(1);
            continue;
        }
        
        if steps > highest_steps {
            highest_steps = steps;
            println!("Number: {}, Steps: {}", &number.separate_with_commas(), &highest_steps.separate_with_commas());
            if let Err(err) = write_to_file("collatz_data.csv", &number, &highest_steps) {
                eprintln!("Failed to write file: {}", err);
                std::process::exit(1);
            }
        }
        number = number.add(1);
    }
}