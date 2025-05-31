use std::io;

fn main() {
    println!("Enter the nth Fibonacci number that you would like to calculate, i.e 4: ");
    let number: i32 = get_user_number();
    calculate_fibonacci(number);
}

fn get_user_number() -> i32 {
    loop {
        // get the user's input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        // convert input to a number
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Enter a valid number (i.e 4)");
                continue;
            }
        }
    }
}

fn calculate_fibonacci(number: i32) {
    // Intialize variables
    let mut previous_number: f64 = 0.0;
    let mut fibonacci_number: f64 = 1.0;

    // calcualte fibonacci number
    // handle the 0 and 1 cases seperately from the others
    match number {
        0 => fibonacci_number = 0.0,
        1 => fibonacci_number = 1.0,
        _ => {
            for _ in 2..=number {
                let temp = fibonacci_number;
                fibonacci_number += previous_number;
                previous_number = temp;
            }
        }
    }
    // print answer
    println!("Fibonacci number though {number} iterations is {fibonacci_number}");
}
