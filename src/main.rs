#![forbid(unsafe_code)]

mod user_input;
mod collatz;
mod pretty_print;

extern crate num;
use num::BigInt;
use crate::collatz::collatz;

fn main() {
    let exponent: u32 = user_input::get_user_input("Enter a positive integer: ");

    let base: BigInt = BigInt::from(2);
    let user_input_exponential: BigInt = base.pow(exponent) + 1;
    
    let result: BigInt = collatz(&user_input_exponential);
    
    println!("Base: {}, Exponent: {}, Steps: {}", &base, &exponent, result);
    // println!("{} had {} steps", pretty_print::format_result(user_input), pretty_print::format_result(result));
}