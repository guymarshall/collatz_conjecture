#![forbid(unsafe_code)]

mod user_input;
mod collatz;
mod pretty_print;

extern crate num;
use num::BigInt;
use crate::collatz::collatz;

fn main() {
    let user_input: u32 = user_input::get_user_input("Enter a positive integer: ");

    let user_input_exponential: BigInt = BigInt::from(2).pow(user_input) + 1;
    println!("Finished calculating exponential. Calculating collatz...");

    let result: BigInt = collatz(&user_input_exponential);
    
    println!("{} had {} steps", &user_input_exponential, result);
    // println!("{} had {} steps", pretty_print::format_result(user_input), pretty_print::format_result(result));
}