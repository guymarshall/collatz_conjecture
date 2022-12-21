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