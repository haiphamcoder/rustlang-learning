fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    /*
    x = 6; // this is an error because x is immutable
    println!("The value of x is: {}", x);
     */

    let mut y = 5; // mutable variable
    y = 6;
    println!("The value of y is: {}", y);

    // constants are immutable by default
    const TWO_HOURS_IN_SECONDS: u32 = 60 * 60 * 2;
    println!("The value of TWO_HOURS_IN_SECONDS is: {}s", TWO_HOURS_IN_SECONDS);
}
