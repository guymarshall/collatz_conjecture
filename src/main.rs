#![forbid(unsafe_code)]

mod user_input;
mod collatz;
mod pretty_print;

extern crate num;
use num::BigInt;

fn main() {
    let user_input: BigInt = user_input::get_user_input("Enter a positive integer: ");

    let result: BigInt = collatz::collatz(&user_input);
    
    println!("{} had {} steps", user_input, result);
    // println!("{} had {} steps", pretty_print::format_result(user_input), pretty_print::format_result(result));
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