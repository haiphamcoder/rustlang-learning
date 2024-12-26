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
    y = 6; 
    println!("Giá trị của y là: {}", y);

    // Hằng số - luôn immutable và phải khai báo kiểu
    const SECONDS_IN_TWO_HOURS: u32 = 2 * 60 * 60;
    println!("Số giây trong 2 giờ là: {}s", SECONDS_IN_TWO_HOURS);
}
