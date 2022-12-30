#![forbid(unsafe_code)]

extern crate num;
use num::BigInt;

pub fn collatz(number: BigInt) -> BigInt {
    let mut counter: BigInt = BigInt::from(0);
    let mut dereferenced_number: BigInt = number; //needs fixing

    //if even, divide by 2
    //else times by 3 and add 1
    while &dereferenced_number > &BigInt::from(1) {
        counter += 1;
        if &dereferenced_number % BigInt::from(2) == BigInt::from(0) {
            dereferenced_number = dereferenced_number / 2;
        } else {
            dereferenced_number = ((3 * dereferenced_number) + 1) / 2;
        }
    }

    counter
}

/*
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