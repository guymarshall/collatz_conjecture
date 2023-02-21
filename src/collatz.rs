#![forbid(unsafe_code)]

extern crate num;
use num::BigInt;

pub fn collatz(number: &BigInt) -> BigInt {
    let mut counter: BigInt = BigInt::from(0);
    let mut number_clone: BigInt = number.clone();

    //if even, divide by 2
    //else times by 3 and add 1
    while &number_clone > &BigInt::from(1) {
        counter += 1;
        if &number_clone % BigInt::from(2) == BigInt::from(0) {
            number_clone = number_clone / 2;
        } else {
            number_clone = ((3 * number_clone) + 1) / 2;
        }
    }

    counter
}