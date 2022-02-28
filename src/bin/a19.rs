// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn get_total(h: &HashMap<String, usize>) -> usize {
    h.values().fold(0, |acc, v| acc + v)
}

fn print_items(h: &HashMap<String, usize>) {
    for (k, v) in h.iter() {
        match v {
            s if *s > 0 => println!("{}: {} in stock", k, v),
            _ => println!("{}: out of stock!", k),
        }
    }
}

fn main() {
    let mut furn_items = HashMap::new();
    furn_items.insert("Chairs".to_string(), 5);
    furn_items.insert("Beds".to_string(), 3);
    furn_items.insert("Tables".to_string(), 2);
    furn_items.insert("Couches".to_string(), 0);

    print_items(&furn_items);
    println!("there are a total of: {} items", get_total(&furn_items));
}
