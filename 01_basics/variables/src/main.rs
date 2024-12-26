fn main() {
    // Biến không thể thay đổi (immutable)
    let x = 5;
    println!("Giá trị của x là: {}", x);

    /*
    x = 6; // Lỗi vì x là immutable
    println!("Giá trị của x là: {}", x);
     */

    // Biến có thể thay đổi (mutable)
    let mut y = 5;
    y = y + 1;
    println!("Giá trị của y là: {}", y);

    // Hằng số - luôn immutable và phải khai báo kiểu
    const SECONDS_IN_TWO_HOURS: u32 = 2 * 60 * 60;
    println!("Số giây trong 2 giờ là: {}s", SECONDS_IN_TWO_HOURS);

    // Shadowing
    // Shadowing là khái niệm cho phép khai báo lại biến với cùng tên
    let z = 5;
    println!("Giá trị của z là: {}", z);
    {
        let z = z + 1; // z is shadowed here
        println!("Giá trị của z trong block là: {}", z);
    }
    println!("Giá trị của z sau block là: {}", z);

    let mut spaces = "   ";
    /*
    spaces = spaces.len(); // Lỗi vì không cùng kiểu dữ liệu
    println!("Số khoảng trắng là: {}", spaces);
     */

    let spaces = spaces.len(); // Không có lỗi vì spaces đã được khai báo lại
    println!("Số khoảng trắng là: {}", spaces);
}
