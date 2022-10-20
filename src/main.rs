mod user_input;

use std::io;

fn collatz(mut number: u128) -> u128 {
    let mut counter: u128 = 0;
    
    //if even, divide by 2
    //else times by 3 and add 1
    while number > 1 {
        counter += 1;
        if number % 2 == 0 {
            number = number / 2;
        } else {
            number = (3 * number) + 1;
        }
    }

    return counter;
}

fn main() {
    loop {
        let user_input: u128 = user_input::get_user_input("Enter a positive integer: ");

        let mut max_number: u128 = 0;
        let mut max_steps: u128 = 0;

        for i in 1..=user_input {
            if max_steps < collatz(i) {
                max_steps = collatz(i);
                max_number = i;
            }

            println!("{}: {}", i, collatz(i));
        }

        println!("{} had the maximum number of steps at {}", max_number, max_steps);
        break;
    }
}