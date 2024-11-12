use std::io::{self, Write};

fn main() {
    println!("Welcome to the Rust Calculator!");

    loop {
        display_menu();
        
        let mut operation = String::new();
        io::stdin()
            .read_line(&mut operation)
            .expect("Failed to read line");

        // Handle invalid input
        let operation: u32 = match operation.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number.");
                continue;
            }
        };

        if operation == 0 {
            println!("\nExiting...");
            break;
        }

        display_result_header();

        match operation {
            1 => {
                let (num1, num2) = get_two_numbers();
                println!("| Operation: Addition               |");
                println!("| Result: {:<31} |", num1 + num2);
            }
            2 => {
                let (num1, num2) = get_two_numbers();
                println!("| Operation: Subtraction            |");
                println!("| Result: {:<31} |", num1 - num2);
            }
            3 => {
                let (num1, num2) = get_two_numbers();
                println!("| Operation: Multiplication         |");
                println!("| Result: {:<31} |", num1 * num2);
            }
            4 => {
                let (num1, num2) = get_two_numbers();
                println!("| Operation: Division               |");
                if num2 == 0.0 {
                    println!("| Error: Cannot divide by zero      |");
                } else {
                    println!("| Result: {:<31} |", num1 / num2);
                }
            }
            5 => {
                println!("Enter the number you want to square root: ");
                let num = read_number();
                println!("| Operation: Square Root            |");
                if num < 0.0 {
                    println!("| Error: Cannot calculate square root of a negative number |");
                } else {
                    println!("| Result: {:<31} |", num.sqrt());
                }
            }
            _ => println!("| Invalid operation, please enter a number between 0 and 5. |"),
        }
        
        display_result_footer();
    }
}

// Function to display the main menu
fn display_menu() {
    println!("\n+-----------------------------------+");
    println!("| Choose the operation to perform   |");
    println!("+-----------------------------------+");
    println!("| 1. Addition                       |");
    println!("| 2. Subtraction                    |");
    println!("| 3. Multiplication                 |");
    println!("| 4. Division                       |");
    println!("| 5. Square root                    |");
    println!("| 0. Exit                           |");
    println!("+-----------------------------------+");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
}

// Function to display the result header
fn display_result_header() {
    println!("\n+-----------------------------------+");
    println!("|            Result                 |");
    println!("+-----------------------------------+");
}

// Function to display the result footer
fn display_result_footer() {
    println!("+-----------------------------------+\n");
}

// Function to get two numbers from the user
fn get_two_numbers() -> (f64, f64) {
    println!("Enter the first number: ");
    let num1 = read_number();

    println!("Enter the second number: ");
    let num2: f64 = read_number();
    return (num1, num2);
}

// Function to read a number from the user
fn read_number() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input, please enter a number."),
        }
    }
}
