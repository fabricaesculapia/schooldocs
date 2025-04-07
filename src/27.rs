// Exercise 1: Basic Variables and Printing

let name = "John";
println!("Hello, {}!", name);

// Exercise 2: Using Functions

fn main() {
    println!("Hello, {}", greeting("Alice"));
}

fn greeting(name: &str) -> String {
    format!("Hello, {}", name)
}

// Exercise 3: If Statements and Conditionals

let age = 18;
if age >= 18 {
    println!("You are an adult.");
} else {
    println!("You are a minor.");
}

// Exercise 4: Loops

fn main() {
    for i in 0..=5 {
        if i == 3 || i == 6 {
            break; // Exit the loop
        }
        println!("{}", i);
    }
}
