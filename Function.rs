fn main(){
    greet_user("Alice", 32);
    let area = calculateArea(10, 20);
    println!("Area is = {}", area);
}

fn greet(){
    println!("Hello Alice");
     
}

fn greet_user(name: &str, age: i32){
    println!("Hello {}, you are {} years old", name, age);
}

fn calculateArea(length: i32, breadth: i32) -> i32 {
    //return length*breadth;
    length*breadth
    // expression-oriented programming
}