#![forbid(unsafe_code)]

mod user_input;
mod collatz;
mod pretty_print;

extern crate num;
use num::BigInt;

fn main() {
    let user_input: BigInt = user_input::get_user_input("Enter a positive integer: ");
    let printable_user_input: BigInt = user_input.clone();

    let result: BigInt = collatz::collatz(user_input);
    
    println!("{} had {} steps", printable_user_input, result);
    // println!("{} had {} steps", pretty_print::format_result(user_input), pretty_print::format_result(result));
}