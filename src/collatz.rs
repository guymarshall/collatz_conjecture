#![forbid(unsafe_code)]

extern crate num;
use num::BigInt;

pub fn collatz(number: &BigInt) -> BigInt {
    let mut counter: BigInt = BigInt::from(0);
    let mut dereferenced_number: BigInt = number.clone();

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