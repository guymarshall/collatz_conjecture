use std::io;

fn collatz(mut number: i32) -> i32 {
    let mut counter: i32 = 0;
    
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
        println!("Enter a positive integer: ");

        let mut user_input: String = String::new();

        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        let user_input: i32 = match user_input.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };
        println!("You entered {}", user_input);

        let mut max_number: i32 = 0;
        let mut max_steps: i32 = 0;

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