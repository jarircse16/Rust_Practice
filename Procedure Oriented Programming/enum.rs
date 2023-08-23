enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let selected_color = Color::Green;
    match selected_color {
        Color::Red => println!("It's red!"),
        Color::Green => println!("It's green!"),
        Color::Blue => println!("It's blue!"),
    }
}
