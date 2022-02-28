// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

#[derive(Debug)]
enum Flavors {
    Cherry,
    Grape,
}

struct Drink {
    flavor: Flavors,
    capacity: f32,
}

fn print_drink(d: Drink) {
    println!("You ordered a {:?} {:?} drink!", d.capacity, d.flavor);
}

fn main() {
    let my_drink = Drink {
        flavor: Flavors::Cherry,
        capacity: 64.0,
    };

    print_drink(my_drink);
}
