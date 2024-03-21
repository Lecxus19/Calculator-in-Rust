

// CALCULATOR IN RUST 
// MARCH 09 2024 

use std::io;

fn main() {
  println!("Welcome to Rust Calculator");
  println!("-----------------------------------");

  println!("Enter first Number: ");
  let mut num1 = String::new();
  io::stdin().read_line(&mut num1).expect("Failed to read line");
  let num1: f64 = num1.trim().parse().expect("Please Enter a numnber");

  println!("Enter Second Number: ");
  let mut num2 = String::new();
  io::stdin().read_line(&mut num2).expect("Failed to read line");
  let num2: f64 = num2.trim().parse().expect("Please Enter a numnber");

  println!("Enter Operation (+, -, *, /): ");
  let mut operation = String::new();
  io::stdin().read_line(&mut operation).expect("Failed to read line");
  let operation = operation.trim(); 

  let result = match operation {
    "+" => num1  + num2,
    "-" => num1  - num2,
    "*" => num1  * num2,
    "/" => {
        if num2 != 0.0 {
            num1 / num2 
        } else {
           panic!("Error: Division by Zero");
        }
    },
    _ => {
        panic!("Invalid Operation");
     }
  };
  println!("Result: {}", result);
}

