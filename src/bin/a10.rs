// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn print_message(msg: &str) {
    println!("{}", msg);
}

fn main() {
    let var_under_test: usize = 4000;
    let to_print_big: bool = match var_under_test {
        v if v > 100 => true,
        _ => false,
    };

    if to_print_big {
        print_message("it's big");
    } else {
        print_message("it's small");
    }
}
