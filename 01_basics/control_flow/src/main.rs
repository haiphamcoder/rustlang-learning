fn main() {
    let number = 10;

    // if statement
    if number < 5 {
        println!("number is less than 5");
    } else {
        println!("number is greater or equal to 5");
    }

    // if let statement
    let some_number = Some(5);
    if let Some(5) = some_number {
        println!("number is 5");
    }

    let number = if number < 5 {
        10
    } else {
        20
    };
    println!("number is {}", number);
}
