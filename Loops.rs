fn main(){
    // let mut counter = 0;
    
    // loop {
    //     counter += 1;
    //     println!("Counter: {}", counter);

    //     if counter >= 5 {
    //         break;
    //     }
    // }

    // let result = loop {
    //     counter +=  1;
    //     if counter == 10{
    //         break counter*2;
    //     }
    // };

    // println!("Result: {}", result);

    // let mut number = 3;

    // while number != 0 {
    //     println!("{}!", number);
    //     number -= 1;
    // }

    // println!("out of loop now");

    // let arr = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("Value: {}", arr[index]);
    //     index += 1;
    // }

    let arr = [10, 20, 30, 40, 50];

    for element in arr.iter() {
        println!("Element: {}", element);
    }

    for number in 1..=5 {
        println!("Number: {}", number);
    }

    for number in (1..=5).rev() {
        // 5..1
        println!("Reverse Number: {}", number);
    }
    
}