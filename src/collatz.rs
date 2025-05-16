extern crate num;

use num::{BigInt, Integer};

pub fn collatz(number: &BigInt) -> BigInt {
    let mut counter: BigInt = BigInt::from(1);
    let mut number_clone: BigInt = number.clone();

    number_clone = ((3 * number_clone) + 1) / 2;

    // if even, divide by 2
    // else times by 3 and add 1
    while number_clone > BigInt::from(1) {
        counter += 1;
        if number_clone.is_even() {
            number_clone /= 2;
        } else {
            number_clone = ((3 * number_clone) + 1) / 2;
        }
    }

    counter
}
