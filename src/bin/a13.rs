// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

fn main() {
    let my_vec: Vec<u8> = vec![10, 20, 30, 40];

    for n in &my_vec {
        match n {
            30 => println!("thirty"),
            _ => println!("{}", n),
        }
    }

    println!("The len of my_vec is: {}", my_vec.len());
}
