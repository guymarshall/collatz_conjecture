mod collatz;
mod file_utilities;

extern crate num;

use crate::collatz::collatz;
use crate::file_utilities::{read_last_values, write_to_file};
use num::BigInt;
use std::ops::Add;
use std::str::FromStr;
use thousands::Separable;

fn main() {
    let mut number: BigInt = BigInt::from_str("1").unwrap();
    let mut highest_steps: BigInt = BigInt::from_str("0").unwrap();

    if let Ok(result) = read_last_values("collatz_data.csv") {
        number = result[0].clone();
        highest_steps = result[1].clone();
    }

    loop {
        let steps: BigInt = collatz(&number);

        if steps > highest_steps {
            highest_steps = steps;
            println!(
                "Number: {}, Steps: {}",
                &number.separate_with_commas(),
                &highest_steps.separate_with_commas()
            );
            if let Err(why) = write_to_file("collatz_data.csv", &number, &highest_steps) {
                eprintln!("Failed to write file: {}", why);
                std::process::exit(1);
            }
        }
        number = number.add(2);
    }
}
