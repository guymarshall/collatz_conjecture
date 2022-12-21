#![forbid(unsafe_code)]

mod user_input;
mod collatz;

fn main() {
    let user_input: i128 = user_input::get_user_input("Enter a positive integer: ");

    let mut max_number: i128 = 0;
    let mut max_steps: i128 = 0;

    for i in 1..=user_input {
        let result: i128 = collatz::collatz(i);
        if max_steps < result {
            max_steps = result;
            max_number = i;
        }
    }

    println!("{} had the maximum number of steps at {}", max_number, max_steps);
}