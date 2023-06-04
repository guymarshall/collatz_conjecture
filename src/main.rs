#![forbid(unsafe_code)]

mod user_input;
mod collatz;

extern crate num;
use num::BigInt;
use crate::collatz::collatz;

fn main() {
    let exponent: BigInt = user_input::input("Enter a positive integer: ");

    let result: BigInt = collatz(&exponent);

    println!("Number: {}, Steps: {}", &exponent, result);
}