use round::round;
use std::io;

fn main() {
    // print the user a message
    println!("Are you inputting Fahrenheit or Celsius? (F/C)");
    // get user input
    let unit: char = get_unit();
    // print the user a message
    println!("Enter the temperature: ");
    // get user input
    let temp: f32 = get_temp();
    // convert
    if unit == 'F' {
        convert_to_celcius(temp);
    } else {
        convert_to_farenheit(temp);
    }
}

fn get_unit() -> char {
    loop {
        // Initialize variables and get user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // convert string to char .next is needed to actual turn the chars iterator to a char
        // and unwarp_or deals with the empty input case
        let unit: char = input.trim().to_uppercase().chars().next().unwrap_or(' ');

        // Check for valid units F and C
        if unit == 'F' || unit == 'C' {
            return unit;
        }
        println!("Please enter a valid unit (F or C):")
    }
}

fn get_temp() -> f32 {
    loop {
        // Intialize variables and grab user input
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        // Convert the input inot a float, if that cannot happen then make sure
        // the use inputs a valid number
        match temp.trim().parse::<f32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Please enter a valid number:");
                continue;
            }
        }
    }
}

fn convert_to_celcius(temp: f32) {
    let temp_c: f32 = round(((temp as f64) - 32.0) * (5.0 / 9.0), 1) as f32;
    println!("{temp}F converts to {temp_c}C");
}

fn convert_to_farenheit(temp: f32) {
    let temp_f: f32 = round((temp as f64) * (9.0 / 5.0) + 32.0, 1) as f32;
    println!("{temp}C converts to {temp_f}F");
}
