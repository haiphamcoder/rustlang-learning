fn main() {
    println!("Basic function calls:");
    print_hello();

    let x = 5;
    let y = 10;
    let sum = add_numbers(x, y);
    println!("{} + {} = {}", x, y, sum);

    let formatted = format_greeting("Alice");
    println!("{}", formatted);
}

fn print_hello() {
    println!("Hello from a function!");
}

fn add_numbers(x: i32, y: i32) -> i32 {
    x + y // Expression returns value
}

fn format_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}
