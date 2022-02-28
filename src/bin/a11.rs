// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Item {
    id: i32,
    qty: i32,
}

fn get_vals(i: &Item) -> (String, String) {
    (format!("{}", i.qty), format!("{}", i.id))
}

fn main() {
    let my_item = Item { id: 123, qty: 20 };
    let (q, i) = get_vals(&my_item);
    println!("my item has qty: {} and id: {}", q, i);
}
