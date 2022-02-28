// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
    Black,
    White,
}

fn main() {
    let my_color: Color = Color::Blue;
    match my_color {
        Color::Black => println!("it's black, the darkest of dark"),
        Color::White => println!("it's white, the lightedt of light"),
        _ => println!("it's {:?}, a boring color to be sure", my_color),
    }
}
