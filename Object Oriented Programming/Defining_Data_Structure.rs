struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 20,
    };

    println!("Rectangle width: {}, height: {}", rect.width, rect.height);
}
