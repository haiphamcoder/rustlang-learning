fn main() {
    let s1 = String::from("Hello, world!");
    print!("s1: {}\n", s1);
    let s2 = s1; // Move s1 to s2 (s1 is no longer valid)
    print!("s2: {}\n", s2);
    // print!("s1: {}\n", s1); // Error: value used here after move

    // Clone trait, for heap data
    let s3 = String::from("Welcome to Rust!");
    let s4 = s3.clone();
    print!("s3: {}\n", s3);
    print!("s4: {}\n", s4);

    // Copy trait, only for primitive types
    let x = 5;
    let y = x;
    print!("x: {}\n", x);
    print!("y: {}\n", y);
}
