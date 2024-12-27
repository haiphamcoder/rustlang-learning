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

    // loop
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 10 {
            break;
        }
    }

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for loop
    for number in 1..4 {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    // for loop with range
    for number in 0..=5 {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!"); // 0 1 2 3 4 5

    // for loop with range and step
    for number in (0..=5).step_by(2) {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!"); // 0 2 4
}
