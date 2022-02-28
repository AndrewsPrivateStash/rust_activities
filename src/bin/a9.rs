// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

struct Coord {
    x: f64,
    y: f64,
}

fn make_tuple(x: f64, y: f64) -> (f64, f64) {
    (x, y)
}

fn match_choice(y_val: f64) {
    match y_val {
        y if y < 5.0 => println!("y less then 5"),
        y if y > 5.0 => println!("y greater then 5"),
        _ => println!("y equal to 5"),
    }
}

fn print_out_struct(c: &Coord) {
    match_choice(c.y);
    println!("no one cares about the x_val {}!", c.x);
}

fn print_tuple(t: (f64, f64)) {
    let (x, y) = t;
    match_choice(y);
    println!("no one cares about the x_val {}!", x);
}

fn main() {
    let crd = Coord { x: 1.0, y: 2.0 };
    print_out_struct(&crd);
    println!("and now for the tuple..");
    let tup = make_tuple(crd.x, crd.y);
    print_tuple(tup);
}
