fn main() {
    let number = 3;

    match number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("default case"),
    }

    let description = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        4..=9 => "small number",
        2 | 3 | 5 | 7 | 11 => "prime number",
        _ => "default case"
    };

    println!("{}", description);
}