mod user_input;

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
    let user_input: u128 = user_input::get_user_input("Enter a positive integer: ");
    let update_frequency: u128 = user_input::get_user_input("Print update every: ");

    let mut max_number: u128 = 0;
    let mut max_steps: u128 = 0;

    for i in 1..=user_input {
        let result = collatz(i);
        if max_steps < result {
            max_steps = result;
            max_number = i;
        }

        if i % update_frequency == 0 {
            println!("{}: {}", i, result);
        }

        // println!("{}: {}", i, collatz(i));
    }

    println!("{} had the maximum number of steps at {}", max_number, max_steps);
}