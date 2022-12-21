#![forbid(unsafe_code)]

pub fn collatz(mut number: i128) -> i128 {
    let mut counter: i128 = 0;
    
    //if even, divide by 2
    //else times by 3 and add 1
    while number > 1 {
        counter += 1;
        if number % 2 == 0 {
            number = number / 2;
        } else {
            number = ((3 * number) + 1) / 2;
        }
    }

    return counter;
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