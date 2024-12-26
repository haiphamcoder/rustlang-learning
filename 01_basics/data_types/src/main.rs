fn main() {
    /*
    Scalar types: đại diện cho một giá trị duy nhất
    - Integer types: số nguyên
    - Floating-point types: số thực
    - Boolean types: true hoặc false
    - Character types: ký tự
     */

    let integer_var: i32 = 5;
    let float_var: f64 = 5.0;
    let boolean_var: bool = true;
    let character_var: char = '😀';

    println!("integer_var: {}", integer_var);
    println!("float_var: {}", float_var);
    println!("boolean_var: {}", boolean_var);
    println!("character_var: {}", character_var);
    println!();

    /*  
    Compound types: đại diện cho nhiều giá trị
    - Tuple: là một cách chung để nhóm một số giá trị lại với nhau (các giá trị có thể khác kiểu dữ liệu)
    - Array: là một cách để tập hợp nhiều giá trị lại với nhau (các giá trị có cùng kiểu dữ liệu)
     */

    let tuple_var: (i32, f64, bool, char) = (6, 6.0, false, 'A');
    let (integer_var, float_var, boolean_var, character_var) = tuple_var;
    println!("integer_var: {}", integer_var);
    println!("float_var: {}", float_var);
    println!("boolean_var: {}", boolean_var);
    println!("character_var: {}", character_var);
    println!();
    
    let array_var: [i32; 5] = [13, 23, 31, 47, 59];
    println!("first: {}", array_var[0]);
    println!("second: {}", array_var[1]);
    println!("third: {}", array_var[2]);
    println!("fourth: {}", array_var[3]);
    println!("fifth: {}", array_var[4]);
    println!();

}
