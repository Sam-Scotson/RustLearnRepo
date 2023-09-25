// Learning the basic's of Rust

use std::io;

fn main() {
    // Variables & Data Types
    let number: i32 = 42;
    let greeting: &str = "Hello, Rust!";
    let is_true: bool = true;

    println!("Number: {}", number);
    println!("Greeting: {}", greeting);
    println!("Is true: {}", is_true);

    // User Input
    println!("Enter your name:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    println!("Hello, {}!", input.trim());

    // if-else statements
    let x = 5;
    if x > 0 {
        println!("x is positive");
    } else if x < 0 {
        println!("x is negative");
    } else {
        println!("x is zero");
    }

    // for and while statements 
    for i in 1..=5 {
        println!("Iteration: {}", i);
    }

    let mut counter = 0;
    while counter < 3 {
        println!("Counter: {}", counter);
        counter += 1;
    }

    // Functions
    let result = add(3, 4);
    println!("3 + 4 = {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}