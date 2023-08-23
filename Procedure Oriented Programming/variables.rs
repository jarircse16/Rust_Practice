fn main() {
    let x: i32 = 42; // Immutable variable
    let mut y = 24;  // Mutable variable
    println!("x = {}, y = {}", x, y);
    y = 10;
    println!("Now y = {}", y);
}
