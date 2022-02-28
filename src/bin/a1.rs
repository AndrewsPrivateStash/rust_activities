// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn main() {
    println!("{} {}", display_first(), display_last());
}

fn display_first() -> String {
    "Andrew".to_string()
}

fn display_last() -> String {
    "Pfaendler".to_string()
}
