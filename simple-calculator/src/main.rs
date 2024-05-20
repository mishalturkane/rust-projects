use std::io;

// Define the Operation enum with variants for each arithmetic operation
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Function to perform the calculation based on the Operation enum variant
fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => {
            if b != 0.0 {
                a / b
            } else {
                panic!("Cannot divide by zero")
            }
        }
    }
}

fn main() {
    let mut input = String::new();

    // Prompt for the first number
    println!("Enter the first number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number1: f64 = input.trim().parse().expect("Please enter a valid number");

    // Prompt for the operation
    input.clear();
    println!("Enter the operation (+, -, *, /):");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let operation = input.trim().to_string();

    // Prompt for the second number
    input.clear();
    println!("Enter the second number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number2: f64 = input.trim().parse().expect("Please enter a valid number");

    // Create an Operation enum instance with the parsed input values
    let operation = match operation.as_str() {
        "+" => Operation::Add(number1, number2),
        "-" => Operation::Subtract(number1, number2),
        "*" => Operation::Multiply(number1, number2),
        "/" => Operation::Divide(number1, number2),
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    // Call the calculate function with the created Operation enum instance
    let result = calculate(operation);

    // Print the result to the console
    println!("The result of the operation is: {}", result);
}
