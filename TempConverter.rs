use std::io;

fn main() {
    println!("Temperature Converter");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    
    let choice: u32 = choice.trim().parse().expect("Please enter a number!");
    
    println!("Enter temperature:");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read line");
    
    let temp: f64 = temp.trim().parse().expect("Please enter a number!");
    
    if choice == 1 {
        let fahrenheit = (temp * 9.0/5.0) + 32.0;
        println!("{}°C = {:.1}°F", temp, fahrenheit);
    } else {
        let celsius = (temp - 32.0) * 5.0/9.0;
        println!("{}°F = {:.1}°C", temp, celsius);
    }
}