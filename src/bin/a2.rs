// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn sum_two(x: i32, y: i32) -> i32 {
    x + y
}

fn display_result(in_num: i32) {
    println!("{:?}", in_num)
}

fn main() {
    display_result(sum_two(3, 5));
}
