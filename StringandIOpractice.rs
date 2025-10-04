use std::io;

fn main(){
    let name = "Alice";
    let age = 30;
    let height = 5.7;

    // Alice is of age 30 years and her height 5.7 inch

    println!("{} is of age {} years and her height {} inch", name, age, height);

    // Positional arguments
    println!("{0} is {1} years old. {0} is learning Rust!", name, age);

      // Named arguments
    println!("{name} is {age} years old and {height} feet tall.");

        // Formatting options
    println!("Pi is approximately {:.3}", 3.14159); // 3.142
    println!("Binary: {:b}, Hex: {:x}", 10, 255);

    println!("Enter your name:-");
    let mut name = String::new();

    io::stdin().read_line(&mut name).expect("Failed to read line");

    println!("Hello, {}! Welcome to Rust.", name.trim());
}